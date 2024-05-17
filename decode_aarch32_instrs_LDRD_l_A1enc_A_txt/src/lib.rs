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
use ConditionPassed::*;
use execute_aarch32_instrs_LDRD_l_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_LDRD_l_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    U: bool,
    Rt: u8,
    imm4H: u8,
    imm4L: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        add: bool,
        t: i64,
        imm32: u32,
        t2: i64,
        cond: u8,
        U: bool,
        Rt: u8,
        imm4H: u8,
        imm4L: u8,
    }
    let fn_state = FunctionState {
        cond,
        U,
        Rt,
        imm4H,
        imm4L,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call ConditionPassed(s_0_0)
        let s_0_1: bool = ConditionPassed(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b2 b1
        if s_0_1 {
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
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var cond:u8
        let s_2_0: u8 = fn_state.cond;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // C s_2_2: const #15u : u8
        let s_2_2: u8 = 15;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 4u16);
        // D s_2_4: cmp-ne s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) != (s_2_3));
        // N s_2_5: assert s_2_4
        let s_2_5: () = assert!(s_2_4);
        // C s_2_6: const #0s : i
        let s_2_6: i128 = 0;
        // D s_2_7: read-var Rt:u8
        let s_2_7: u8 = fn_state.Rt;
        // D s_2_8: cast zx s_2_7 -> bv
        let s_2_8: Bits = Bits::new(s_2_7 as u128, 4u16);
        // C s_2_9: const #1u : u64
        let s_2_9: u64 = 1;
        // D s_2_10: bit-extract s_2_8 s_2_6 s_2_9
        let s_2_10: Bits = (Bits::new(
            ((s_2_8) >> (s_2_6)).value(),
            u16::try_from(s_2_9).unwrap(),
        ));
        // D s_2_11: cast reint s_2_10 -> u8
        let s_2_11: bool = ((s_2_10.value()) != 0);
        // C s_2_12: const #0s : i
        let s_2_12: i128 = 0;
        // C s_2_13: const #0u : u64
        let s_2_13: u64 = 0;
        // D s_2_14: cast zx s_2_11 -> u64
        let s_2_14: u64 = (s_2_11 as u64);
        // C s_2_15: const #1u : u64
        let s_2_15: u64 = 1;
        // D s_2_16: and s_2_14 s_2_15
        let s_2_16: u64 = ((s_2_14) & (s_2_15));
        // D s_2_17: cmp-eq s_2_16 s_2_15
        let s_2_17: bool = ((s_2_16) == (s_2_15));
        // D s_2_18: lsl s_2_14 s_2_12
        let s_2_18: u64 = s_2_14 << s_2_12;
        // D s_2_19: or s_2_13 s_2_18
        let s_2_19: u64 = ((s_2_13) | (s_2_18));
        // D s_2_20: cmpl s_2_18
        let s_2_20: u64 = !s_2_18;
        // D s_2_21: and s_2_13 s_2_20
        let s_2_21: u64 = ((s_2_13) & (s_2_20));
        // D s_2_22: select s_2_17 s_2_19 s_2_21
        let s_2_22: u64 = if s_2_17 { s_2_19 } else { s_2_21 };
        // D s_2_23: cast trunc s_2_22 -> u8
        let s_2_23: bool = ((s_2_22) != 0);
        // D s_2_24: cast zx s_2_23 -> bv
        let s_2_24: Bits = Bits::new(s_2_23 as u128, 1u16);
        // C s_2_25: const #1u : u8
        let s_2_25: bool = true;
        // C s_2_26: cast zx s_2_25 -> bv
        let s_2_26: Bits = Bits::new(s_2_25 as u128, 1u16);
        // D s_2_27: cmp-eq s_2_24 s_2_26
        let s_2_27: bool = ((s_2_24) == (s_2_26));
        // N s_2_28: branch s_2_27 b6 b3
        if s_2_27 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var Rt:u8
        let s_3_0: u8 = fn_state.Rt;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 4u16);
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // D s_3_4: write-var t <= s_3_3
        fn_state.t = s_3_3;
        // C s_3_5: const #1s : i
        let s_3_5: i128 = 1;
        // D s_3_6: read-var t:i64
        let s_3_6: i64 = fn_state.t;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: add s_3_7 s_3_5
        let s_3_8: i128 = (s_3_7 + s_3_5);
        // D s_3_9: cast reint s_3_8 -> i64
        let s_3_9: i64 = (s_3_8 as i64);
        // D s_3_10: write-var t2 <= s_3_9
        fn_state.t2 = s_3_9;
        // D s_3_11: read-var imm4H:u8
        let s_3_11: u8 = fn_state.imm4H;
        // D s_3_12: cast zx s_3_11 -> bv
        let s_3_12: Bits = Bits::new(s_3_11 as u128, 4u16);
        // D s_3_13: read-var imm4L:u8
        let s_3_13: u8 = fn_state.imm4L;
        // D s_3_14: cast zx s_3_13 -> bv
        let s_3_14: Bits = Bits::new(s_3_13 as u128, 4u16);
        // D s_3_15: cast reint s_3_12 -> u128
        let s_3_15: u128 = (s_3_12.value() as u128);
        // D s_3_16: size-of s_3_12
        let s_3_16: u16 = s_3_12.length();
        // D s_3_17: cast reint s_3_14 -> u128
        let s_3_17: u128 = (s_3_14.value() as u128);
        // D s_3_18: size-of s_3_14
        let s_3_18: u16 = s_3_14.length();
        // D s_3_19: lsl s_3_15 s_3_18
        let s_3_19: u128 = s_3_15 << s_3_18;
        // D s_3_20: or s_3_19 s_3_17
        let s_3_20: u128 = ((s_3_19) | (s_3_17));
        // D s_3_21: add s_3_16 s_3_18
        let s_3_21: u16 = (s_3_16 + s_3_18);
        // D s_3_22: create-bits s_3_20 s_3_21
        let s_3_22: Bits = Bits::new(s_3_20, s_3_21);
        // D s_3_23: cast reint s_3_22 -> u8
        let s_3_23: u8 = (s_3_22.value() as u8);
        // C s_3_24: const #32s : i
        let s_3_24: i128 = 32;
        // D s_3_25: cast zx s_3_23 -> bv
        let s_3_25: Bits = Bits::new(s_3_23 as u128, 8u16);
        // D s_3_26: bits-cast zx s_3_25 -> bv length s_3_24
        let s_3_26: Bits = s_3_25.zero_extend(s_3_24);
        // D s_3_27: cast reint s_3_26 -> u32
        let s_3_27: u32 = (s_3_26.value() as u32);
        // D s_3_28: write-var imm32 <= s_3_27
        fn_state.imm32 = s_3_27;
        // D s_3_29: read-var U:u8
        let s_3_29: bool = fn_state.U;
        // D s_3_30: cast zx s_3_29 -> bv
        let s_3_30: Bits = Bits::new(s_3_29 as u128, 1u16);
        // C s_3_31: const #1u : u8
        let s_3_31: bool = true;
        // C s_3_32: cast zx s_3_31 -> bv
        let s_3_32: Bits = Bits::new(s_3_31 as u128, 1u16);
        // D s_3_33: cmp-eq s_3_30 s_3_32
        let s_3_33: bool = ((s_3_30) == (s_3_32));
        // D s_3_34: write-var add <= s_3_33
        fn_state.add = s_3_33;
        // C s_3_35: const #15s : i
        let s_3_35: i128 = 15;
        // D s_3_36: read-var t2:i64
        let s_3_36: i64 = fn_state.t2;
        // D s_3_37: cast zx s_3_36 -> i
        let s_3_37: i128 = (i128::try_from(s_3_36).unwrap());
        // D s_3_38: cmp-eq s_3_37 s_3_35
        let s_3_38: bool = ((s_3_37) == (s_3_35));
        // N s_3_39: branch s_3_38 b5 b4
        if s_3_38 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var add:u8
        let s_4_0: bool = fn_state.add;
        // D s_4_1: read-var imm32:u32
        let s_4_1: u32 = fn_state.imm32;
        // D s_4_2: read-var t:i64
        let s_4_2: i64 = fn_state.t;
        // D s_4_3: read-var t2:i64
        let s_4_3: i64 = fn_state.t2;
        // D s_4_4: call execute_aarch32_instrs_LDRD_l_Op_A_txt(s_4_0, s_4_1, s_4_2, s_4_3)
        let s_4_4: () = execute_aarch32_instrs_LDRD_l_Op_A_txt(
            state,
            tracer,
            s_4_0,
            s_4_1,
            s_4_2,
            s_4_3,
        );
        // N s_4_5: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: panic
        panic!("{:?}", ());
        // N s_5_1: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: panic
        panic!("{:?}", ());
        // N s_6_1: return
        return;
    }
}
