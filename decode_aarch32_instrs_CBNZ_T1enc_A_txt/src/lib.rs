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
use place_slice::*;
use InITBlock::*;
use execute_aarch32_instrs_CBNZ_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_CBNZ_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    op: bool,
    i: bool,
    imm5: u8,
    Rn: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        imm32: u32,
        nonzero: bool,
        n: i64,
        op: bool,
        i: bool,
        imm5: u8,
        Rn: u8,
    }
    let fn_state = FunctionState {
        op,
        i,
        imm5,
        Rn,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var Rn:u8
        let s_0_0: u8 = fn_state.Rn;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 3u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: write-var n <= s_0_3
        fn_state.n = s_0_3;
        // D s_0_5: read-var i:u8
        let s_0_5: bool = fn_state.i;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 1u16);
        // D s_0_7: read-var imm5:u8
        let s_0_7: u8 = fn_state.imm5;
        // D s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 5u16);
        // D s_0_9: cast reint s_0_6 -> u128
        let s_0_9: u128 = (s_0_6.value() as u128);
        // D s_0_10: size-of s_0_6
        let s_0_10: u16 = s_0_6.length();
        // D s_0_11: cast reint s_0_8 -> u128
        let s_0_11: u128 = (s_0_8.value() as u128);
        // D s_0_12: size-of s_0_8
        let s_0_12: u16 = s_0_8.length();
        // D s_0_13: lsl s_0_9 s_0_12
        let s_0_13: u128 = s_0_9 << s_0_12;
        // D s_0_14: or s_0_13 s_0_11
        let s_0_14: u128 = ((s_0_13) | (s_0_11));
        // D s_0_15: add s_0_10 s_0_12
        let s_0_15: u16 = (s_0_10 + s_0_12);
        // D s_0_16: create-bits s_0_14 s_0_15
        let s_0_16: Bits = Bits::new(s_0_14, s_0_15);
        // D s_0_17: cast reint s_0_16 -> u8
        let s_0_17: u8 = (s_0_16.value() as u8);
        // C s_0_18: const #32s : i
        let s_0_18: i128 = 32;
        // C s_0_19: const #0s : i
        let s_0_19: i128 = 0;
        // C s_0_20: const #6s : i
        let s_0_20: i128 = 6;
        // C s_0_21: const #1s : i
        let s_0_21: i128 = 1;
        // D s_0_22: cast zx s_0_17 -> bv
        let s_0_22: Bits = Bits::new(s_0_17 as u128, 6u16);
        // D s_0_23: call place_slice(s_0_18, s_0_22, s_0_19, s_0_20, s_0_21)
        let s_0_23: Bits = place_slice(
            state,
            tracer,
            s_0_18,
            s_0_22,
            s_0_19,
            s_0_20,
            s_0_21,
        );
        // D s_0_24: cast reint s_0_23 -> u32
        let s_0_24: u32 = (s_0_23.value() as u32);
        // D s_0_25: write-var imm32 <= s_0_24
        fn_state.imm32 = s_0_24;
        // D s_0_26: read-var op:u8
        let s_0_26: bool = fn_state.op;
        // D s_0_27: cast zx s_0_26 -> bv
        let s_0_27: Bits = Bits::new(s_0_26 as u128, 1u16);
        // C s_0_28: const #1u : u8
        let s_0_28: bool = true;
        // C s_0_29: cast zx s_0_28 -> bv
        let s_0_29: Bits = Bits::new(s_0_28 as u128, 1u16);
        // D s_0_30: cmp-eq s_0_27 s_0_29
        let s_0_30: bool = ((s_0_27) == (s_0_29));
        // D s_0_31: write-var nonzero <= s_0_30
        fn_state.nonzero = s_0_30;
        // C s_0_32: const #() : ()
        let s_0_32: () = ();
        // S s_0_33: call InITBlock(s_0_32)
        let s_0_33: bool = InITBlock(state, tracer, s_0_32);
        // N s_0_34: branch s_0_33 b2 b1
        if s_0_33 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var imm32:u32
        let s_1_0: u32 = fn_state.imm32;
        // D s_1_1: read-var n:i64
        let s_1_1: i64 = fn_state.n;
        // D s_1_2: read-var nonzero:u8
        let s_1_2: bool = fn_state.nonzero;
        // D s_1_3: call execute_aarch32_instrs_CBNZ_Op_A_txt(s_1_0, s_1_1, s_1_2)
        let s_1_3: () = execute_aarch32_instrs_CBNZ_Op_A_txt(
            state,
            tracer,
            s_1_0,
            s_1_1,
            s_1_2,
        );
        // N s_1_4: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: panic
        panic!("{:?}", ());
        // N s_2_1: return
        return;
    }
}
