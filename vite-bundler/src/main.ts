import { become_supporter } from "rust-wasm";

const supporter = become_supporter("Ken", 2022);

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
<p>${supporter.say()}</p>
`;
