#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use IsZeroBit::*;
use R_read::*;
use common::*;
pub fn execute_aarch32_instrs_TEQ_i_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    carry: bool,
    imm32: u32,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        carry: bool,
        imm32: u32,
        n: i64,
    }
    let fn_state = FunctionState {
        carry,
        imm32,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var n:i64
        let s_0_0: i64 = fn_state.n;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_4: read-var imm32:u32
        let s_0_4: u32 = fn_state.imm32;
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 32u16);
        // D s_0_6: xor s_0_3 s_0_5
        let s_0_6: Bits = ((s_0_3) ^ (s_0_5));
        // D s_0_7: cast reint s_0_6 -> u32
        let s_0_7: u32 = (s_0_6.value() as u32);
        // C s_0_8: const #31s : i
        let s_0_8: i128 = 31;
        // D s_0_9: cast zx s_0_7 -> bv
        let s_0_9: Bits = Bits::new(s_0_7 as u128, 32u16);
        // C s_0_10: const #1u : u64
        let s_0_10: u64 = 1;
        // D s_0_11: bit-extract s_0_9 s_0_8 s_0_10
        let s_0_11: Bits = (Bits::new(
            ((s_0_9) >> (s_0_8)).value(),
            u16::try_from(s_0_10).unwrap(),
        ));
        // D s_0_12: cast reint s_0_11 -> u8
        let s_0_12: bool = ((s_0_11.value()) != 0);
        // C s_0_13: const #0s : i
        let s_0_13: i128 = 0;
        // C s_0_14: const #0u : u64
        let s_0_14: u64 = 0;
        // D s_0_15: cast zx s_0_12 -> u64
        let s_0_15: u64 = (s_0_12 as u64);
        // C s_0_16: const #1u : u64
        let s_0_16: u64 = 1;
        // D s_0_17: and s_0_15 s_0_16
        let s_0_17: u64 = ((s_0_15) & (s_0_16));
        // D s_0_18: cmp-eq s_0_17 s_0_16
        let s_0_18: bool = ((s_0_17) == (s_0_16));
        // D s_0_19: lsl s_0_15 s_0_13
        let s_0_19: u64 = s_0_15 << s_0_13;
        // D s_0_20: or s_0_14 s_0_19
        let s_0_20: u64 = ((s_0_14) | (s_0_19));
        // D s_0_21: cmpl s_0_19
        let s_0_21: u64 = !s_0_19;
        // D s_0_22: and s_0_14 s_0_21
        let s_0_22: u64 = ((s_0_14) & (s_0_21));
        // D s_0_23: select s_0_18 s_0_20 s_0_22
        let s_0_23: u64 = if s_0_18 { s_0_20 } else { s_0_22 };
        // D s_0_24: cast trunc s_0_23 -> u8
        let s_0_24: bool = ((s_0_23) != 0);
        // C s_0_25: const #16984u : u32
        let s_0_25: u32 = 16984;
        // N s_0_26: write-reg s_0_25 <= s_0_24
        let s_0_26: () = {
            state.write_register::<bool>(s_0_25 as isize, s_0_24);
            tracer.write_register(s_0_25 as isize, s_0_24);
        };
        // D s_0_27: cast zx s_0_7 -> bv
        let s_0_27: Bits = Bits::new(s_0_7 as u128, 32u16);
        // D s_0_28: call IsZeroBit(s_0_27)
        let s_0_28: bool = IsZeroBit(state, tracer, s_0_27);
        // C s_0_29: const #16997u : u32
        let s_0_29: u32 = 16997;
        // N s_0_30: write-reg s_0_29 <= s_0_28
        let s_0_30: () = {
            state.write_register::<bool>(s_0_29 as isize, s_0_28);
            tracer.write_register(s_0_29 as isize, s_0_28);
        };
        // D s_0_31: read-var carry:u8
        let s_0_31: bool = fn_state.carry;
        // C s_0_32: const #16971u : u32
        let s_0_32: u32 = 16971;
        // N s_0_33: write-reg s_0_32 <= s_0_31
        let s_0_33: () = {
            state.write_register::<bool>(s_0_32 as isize, s_0_31);
            tracer.write_register(s_0_32 as isize, s_0_31);
        };
        // N s_0_34: return
        return;
    }
}
