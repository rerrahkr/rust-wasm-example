import initWasm, { become_supporter } from "../../rust_wasm/pkg-web/rust_wasm";

initWasm().then(() => {
  const supporter = become_supporter("Amy", 2006);

  document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
<p>${supporter.say()}</p>
`;
});
