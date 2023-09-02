use std::path::PathBuf;
use rnapkin;
use rnapkin::rnamanip::{self, Nucleotide};
use rnapkin::draw::{self, colors::ColorTheme, Mirror};
use rnapkin::forest;
use rnapkin::utils::ParsedInput;



#[tauri::command(rename_all = "snake_case")]
pub fn rnapkin_fn(sequence: &str, height: u32, color_theme: &str, mirror_x: bool, mirror_y: bool, rotation_angle: f64, bubble_radius: f64) -> String {

     let color_theme_rs =  match color_theme.as_ref() {
        "dark" => ColorTheme::dark(), 
        "white" => ColorTheme::white(), 
        "black" => ColorTheme::black(), 
        "bright" => ColorTheme::bright(), 
        "default" => ColorTheme::default(),
        _ =>  ColorTheme::default()
     };


    let mut lines = sequence.split("\n").map(|x| x.to_string());
        
    // https://github.com/ukmrs/rnapkin/blob/main/src/main.rs#L61
    // pi
    // let height = 900;
    // let BUBBLE_RADIUS: f64 = 0.5;     
    // let mut theme = ColorTheme::dark();
    let pi = rnapkin::utils::ParsedInput::parse(&mut lines).unwrap();
    let filename = PathBuf::from("o.x");
    

    let (pairlist, sequence) = match (pi.secondary_structure, pi.sequence) {
        (Some(sst), Some(seq)) => {
            let pl = rnamanip::get_pair_list(&sst);
            let seq = rnamanip::read_sequence(&seq);
            assert_eq!(
                pl.len(),
                seq.len(),
                "sequence and structure have differents lengths!"
            );
            (pl, seq)
        }
        (Some(sst), None) => {
            let pairlist = rnamanip::get_pair_list(&sst);
            let seq = vec![Nucleotide::X; pairlist.len()]; // TODO del XSequence if am not gonna use it
            (pairlist, seq)
        }
        (None, Some(_)) => unimplemented!(
            "Calling external soft e.g. RNAFold to get secondary_structure not yet implemented"
        ),
        (None, None) => panic!("Neither sequence nor secondary structure found in the input file!"),
    };

    let tree = forest::grow_tree(&pairlist);
    let bubbles =
        draw::gather_bubbles(&tree, &sequence, bubble_radius, rotation_angle.to_radians());
    let mirror = Mirror::new(mirror_x, mirror_y);

    let highlights = match pi.highlight {
        Some(hls) => draw::colors::user_input_to_highlight_indices(&hls),
        None => vec![None; sequence.len()],
    };

    let svgout = draw::plot(
        &bubbles,
        bubble_radius,
        &filename,
        &color_theme_rs,
        height,
        mirror,
        &highlights,
    ).unwrap();
    
    svgout.unwrap()

}





#[cfg(test)]
mod tests {
    use super::*;
    use filesize::PathExt;
    use tempfile::NamedTempFile;


    //  Todo:  Test case for malfomred string
    //  Todo:  Allow passing a file instead
    
    #[test]
    fn test_rnapkin() {
        let BUBBLE_RADIUS: f64 = 0.5;
        
        let tree1 = ">fantastic guanine riboswitch
        AAUAUAAUAGGAACACUCAUAUAAUCGCGUGGAUAUGGCACGCAAGUUUCUACCGGGCAC
        ..........(..(.((((.((((..(((((.......)))))..........((((((.
        CGUAAAUGUCCGACUAUGGGUGAGCAAUGGAACCGCACGUGUACGGUUUUUUGUGAUAUC
        ......)))))).....((((((((((((((((((........))))))...........
        AGCAUUGCUUGCUCUUUAUUUGAGCGGGCAAUGCUUUUUUUA
        ..)))))))))))).)))).)))).)..).............";
        
        let mut lines = tree1.split("\n").map(|x| x.to_string());
        
        // https://github.com/ukmrs/rnapkin/blob/main/src/main.rs#L61
        // pi
        let pi = rnapkin::utils::ParsedInput::parse(&mut lines).unwrap();
        let height = 900;

        let filename = PathBuf::from("o.x");
        let mut theme = ColorTheme::dark();
        
        println!("{:?}", pi);
        let (pairlist, sequence) = match (pi.secondary_structure, pi.sequence) {
            (Some(sst), Some(seq)) => {
                let pl = rnamanip::get_pair_list(&sst);
                let seq = rnamanip::read_sequence(&seq);
                assert_eq!(
                    pl.len(),
                    seq.len(),
                    "sequence and structure have differents lenghts!"
                );
                (pl, seq)
            }
            (Some(sst), None) => {
                let pairlist = rnamanip::get_pair_list(&sst);
                let seq = vec![Nucleotide::X; pairlist.len()]; // TODO del XSequence if am not gonna use it
                (pairlist, seq)
            }
            (None, Some(_)) => unimplemented!(
                "Calling external soft e.g. RNAFold to get secondary_structure not yet implemented"
            ),
            (None, None) => panic!("Neither sequence nor secondary structure found in the input file!"),
        };

        let tree = forest::grow_tree(&pairlist);
        let mut bubbles =
            draw::gather_bubbles(&tree, &sequence, BUBBLE_RADIUS, 0.0_f64.to_radians());
        let mirror = Mirror::new(false, false);

        let highlights = match pi.highlight {
            Some(hls) => draw::colors::user_input_to_highlight_indices(&hls),
            None => vec![None; sequence.len()],
        };

        let svgout = draw::plot(
            &bubbles,
            BUBBLE_RADIUS,
            &filename,
            &theme,
            height,
            mirror,
            &highlights,
        ).unwrap();

        println!("{:?}", svgout);
        
        // args


    }
}

