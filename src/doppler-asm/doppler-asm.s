.globl e
e:
  .equ ADMIN_HEADER, 0x0008
  .equ EXPECTED_ADMIN_HEADER, 0x01ff
  .equ ADMIN_KEY_0, 0x0010
  .equ ADMIN_KEY_1, 0x0018
  .equ ADMIN_KEY_2, 0x0020
  .equ ADMIN_KEY_3, 0x0028
  .equ EXPECTED_ADMIN_KEY_0, 0x2222222222222222
  .equ EXPECTED_ADMIN_KEY_1, 0x2222222222222222
  .equ EXPECTED_ADMIN_KEY_2, 0x2222222222222222
  .equ EXPECTED_ADMIN_KEY_3, 0x2222222222222222
  .equ ORACLE_HEADER, 0x2868
  .equ ORACLE_SEQUENCE, 0x28c0 // u64 (8 bytes)
  .equ ORACLE_PRICE, 0x28c8 // u64 (8 bytes)
  .equ INSTRUCTION_SEQUENCE, 0x50e0 // u64 (8 bytes)
  .equ INSTRUCTION_PRICE, 0x50e8 // u64 (8 bytes)

  ldxh r2, [r1+ADMIN_HEADER]
  jne r2, EXPECTED_ADMIN_HEADER, abort

  lddw r2, EXPECTED_ADMIN_KEY_0
  ldxdw r3, [r1+ADMIN_KEY_0]
  jne r2, r3, abort
  
  lddw r2, EXPECTED_ADMIN_KEY_1
  ldxdw r3, [r1+ADMIN_KEY_1]
  jne r2, r3, abort

  lddw r2, EXPECTED_ADMIN_KEY_2 
  ldxdw r3, [r1+ADMIN_KEY_2]
  jne r2, r3, abort

  lddw r2, EXPECTED_ADMIN_KEY_3
  ldxdw r3, [r1+ADMIN_KEY_3]
  jne r2, r3, abort

  ldxdw r2, [r1+INSTRUCTION_SEQUENCE]
  ldxdw r3, [r1+ORACLE_SEQUENCE]
  jgt r2, r3, update

abort:
  mov32 r0, 1
  exit
  
update:
  stxdw [r1+ORACLE_SEQUENCE], r2
  ldxdw r2, [r1+INSTRUCTION_PRICE]
  stxdw [r1+ORACLE_PRICE], r2
  exit