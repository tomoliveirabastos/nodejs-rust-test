import { createRequire } from "module";
const require = createRequire(import.meta.url);
const a = require('.');

const b = a.set_object("Tom Oliveira")
const c = a.get_num_cpus()
const d = a.handle_object({ nome: "Tomzinho" })

console.log(b, c, d)