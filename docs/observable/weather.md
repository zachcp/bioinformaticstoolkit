# Weather report


```js
import {Generators} from "npm:@observablehq/stdlib";
let open = window.__TAURI__.dialog.open;
let invoke = window.__TAURI__.core.invoke;
```


```js
display(2 + Math.random());
```

```js
const nameInput = display(document.createElement("input"));
const name = Generators.input(nameInput);
```

```js
if (name !="") {
    display( await invoke("greet", {name:name}))
}

```

```js
const essay = view(Inputs.textarea({label: "Essay", rows: 6, minlength: 6, submit: true}));
```


```js
let thing = await invoke("greet", {name:essay});
console.log(thing);

```


```js
display(await invoke("greet", {name:essay}))
```

```js
display(await invoke("greet", {name:essay}))
```


```js
thing === null
```



```js
const pending = {
  state: 'pending',
};

function getPromiseState(promise) {
  // We put `pending` promise after the promise to test, 
  // which forces .race to test `promise` first
  return Promise.race([promise, pending]).then(
    (value) => {
      if (value === pending) {
        return value;
      }
      return {
        state: 'resolved',
        value
      };
    },
    (reason) => ({ state: 'rejected', reason })
  );
}

async function delay(milliseconds = 0, returnValue) {
  return new Promise(done => setTimeout((() => done(returnValue)), milliseconds));
}

async function isResolved(promise) {
  return await Promise.race([delay(0, false), promise.then(() => true, () => false)]);
}

console.log(thing.status);

```

