import { become_supporter } from "../rust_wasm/pkg-node";

const supporter = become_supporter("Todd", 2002);

// Todd has been a supporter of Vissel Kobe since 2002!
console.log(supporter.say());
