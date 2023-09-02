use std::path::PathBuf;
use rnapkin;
use rnapkin::rnamanip::{self, Nucleotide};
use rnapkin::draw::{self, colors::ColorTheme, Mirror};
use rnapkin::forest;
use rnapkin::utils::ParsedInput;



#[tauri::command]
pub fn rnapkin_fn(sequence: &str) -> String {

    let mut lines = sequence.split("\n").map(|x| x.to_string());
        
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
    
    svgout

}





#[cfg(test)]
mod tests {
    use super::*;
    use filesize::PathExt;
    use tempfile::NamedTempFile;


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

