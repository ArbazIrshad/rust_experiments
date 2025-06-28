const NUM_ITERATIONS = 1000000000;
let counter = 0;

const start = process.hrtime.bigint();

for (let i = 0; i < NUM_ITERATIONS; i++) {
    counter++;
}

const end = process.hrtime.bigint();
const elapsedNs = end - start;
const elapsedSeconds = Number(elapsedNs) / 1e9;

console.log(`Counter: ${counter}`);
console.log(`Time taken: ${elapsedSeconds} seconds`);
