# doppler-asm

An assembly rewrite of doppler with several extreme optimizations.

1. 20 CUs per call
2. Final binary size: 464 bytes (449 deployable). 
3. Assumes user grinds a pubkey ending in 0x00000000

Created with [sbpf](https://github.com/blueshift-gg/sbpf)