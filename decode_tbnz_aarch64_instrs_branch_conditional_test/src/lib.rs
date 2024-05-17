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
use execute_aarch64_instrs_branch_conditional_test::*;
use common::*;
pub fn decode_tbnz_aarch64_instrs_branch_conditional_test<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rt: u8,
    imm14: u16,
    b40: u8,
    op: bool,
    b5: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        t: i64,
        ga_269904: i64,
        Rt: u8,
        imm14: u16,
        b40: u8,
        op: bool,
        b5: bool,
    }
    let fn_state = FunctionState {
        Rt,
        imm14,
        b40,
        op,
        b5,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var Rt:u8
        let s_0_0: u8 = fn_state.Rt;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 5u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: write-var t <= s_0_3
        fn_state.t = s_0_3;
        // D s_0_5: read-var b5:u8
        let s_0_5: bool = fn_state.b5;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 1u16);
        // C s_0_7: const #1u : u8
        let s_0_7: bool = true;
        // C s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 1u16);
        // D s_0_9: cmp-eq s_0_6 s_0_8
        let s_0_9: bool = ((s_0_6) == (s_0_8));
        // N s_0_10: branch s_0_9 b3 b1
        if s_0_9 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #32s : i64
        let s_1_0: i64 = 32;
        // D s_1_1: write-var ga#269904 <= s_1_0
        fn_state.ga_269904 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#269904:i64
        let s_2_0: i64 = fn_state.ga_269904;
        // D s_2_1: read-var b5:u8
        let s_2_1: bool = fn_state.b5;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 1u16);
        // D s_2_3: read-var b40:u8
        let s_2_3: u8 = fn_state.b40;
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 5u16);
        // D s_2_5: cast reint s_2_2 -> u128
        let s_2_5: u128 = (s_2_2.value() as u128);
        // D s_2_6: size-of s_2_2
        let s_2_6: u16 = s_2_2.length();
        // D s_2_7: cast reint s_2_4 -> u128
        let s_2_7: u128 = (s_2_4.value() as u128);
        // D s_2_8: size-of s_2_4
        let s_2_8: u16 = s_2_4.length();
        // D s_2_9: lsl s_2_5 s_2_8
        let s_2_9: u128 = s_2_5 << s_2_8;
        // D s_2_10: or s_2_9 s_2_7
        let s_2_10: u128 = ((s_2_9) | (s_2_7));
        // D s_2_11: add s_2_6 s_2_8
        let s_2_11: u16 = (s_2_6 + s_2_8);
        // D s_2_12: create-bits s_2_10 s_2_11
        let s_2_12: Bits = Bits::new(s_2_10, s_2_11);
        // D s_2_13: cast reint s_2_12 -> u8
        let s_2_13: u8 = (s_2_12.value() as u8);
        // D s_2_14: cast zx s_2_13 -> bv
        let s_2_14: Bits = Bits::new(s_2_13 as u128, 6u16);
        // D s_2_15: cast zx s_2_14 -> i
        let s_2_15: i128 = (s_2_14.value() as i128);
        // D s_2_16: cast reint s_2_15 -> i64
        let s_2_16: i64 = (s_2_15 as i64);
        // D s_2_17: read-var op:u8
        let s_2_17: bool = fn_state.op;
        // D s_2_18: read-var imm14:u14
        let s_2_18: u16 = fn_state.imm14;
        // D s_2_19: cast zx s_2_18 -> bv
        let s_2_19: Bits = Bits::new(s_2_18 as u128, 14u16);
        // C s_2_20: const #0u : u8
        let s_2_20: u8 = 0;
        // C s_2_21: cast zx s_2_20 -> bv
        let s_2_21: Bits = Bits::new(s_2_20 as u128, 2u16);
        // D s_2_22: cast reint s_2_19 -> u128
        let s_2_22: u128 = (s_2_19.value() as u128);
        // D s_2_23: size-of s_2_19
        let s_2_23: u16 = s_2_19.length();
        // C s_2_24: cast reint s_2_21 -> u128
        let s_2_24: u128 = (s_2_21.value() as u128);
        // D s_2_25: size-of s_2_21
        let s_2_25: u16 = s_2_21.length();
        // D s_2_26: lsl s_2_22 s_2_25
        let s_2_26: u128 = s_2_22 << s_2_25;
        // D s_2_27: or s_2_26 s_2_24
        let s_2_27: u128 = ((s_2_26) | (s_2_24));
        // D s_2_28: add s_2_23 s_2_25
        let s_2_28: u16 = (s_2_23 + s_2_25);
        // D s_2_29: create-bits s_2_27 s_2_28
        let s_2_29: Bits = Bits::new(s_2_27, s_2_28);
        // D s_2_30: cast reint s_2_29 -> u16
        let s_2_30: u16 = (s_2_29.value() as u16);
        // C s_2_31: const #64s : i
        let s_2_31: i128 = 64;
        // D s_2_32: cast zx s_2_30 -> bv
        let s_2_32: Bits = Bits::new(s_2_30 as u128, 16u16);
        // D s_2_33: bits-cast sx s_2_32 -> bv length s_2_31
        let s_2_33: Bits = s_2_32.sign_extend(s_2_31);
        // D s_2_34: cast reint s_2_33 -> u64
        let s_2_34: u64 = (s_2_33.value() as u64);
        // D s_2_35: cast zx s_2_0 -> i
        let s_2_35: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_36: cast reint s_2_35 -> i64
        let s_2_36: i64 = (s_2_35 as i64);
        // D s_2_37: read-var t:i64
        let s_2_37: i64 = fn_state.t;
        // D s_2_38: call execute_aarch64_instrs_branch_conditional_test(s_2_16, s_2_17, s_2_36, s_2_34, s_2_37)
        let s_2_38: () = execute_aarch64_instrs_branch_conditional_test(
            state,
            tracer,
            s_2_16,
            s_2_17,
            s_2_36,
            s_2_34,
            s_2_37,
        );
        // N s_2_39: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #64s : i64
        let s_3_0: i64 = 64;
        // D s_3_1: write-var ga#269904 <= s_3_0
        fn_state.ga_269904 = s_3_0;
        // N s_3_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
