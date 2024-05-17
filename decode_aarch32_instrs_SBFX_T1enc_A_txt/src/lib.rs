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
use execute_aarch32_instrs_SBFX_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_SBFX_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rn: u8,
    imm3: u8,
    Rd: u8,
    imm2: u8,
    widthm1: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_300592: bool,
        lsbit: i64,
        msbit: i64,
        n: i64,
        d: i64,
        Rn: u8,
        imm3: u8,
        Rd: u8,
        imm2: u8,
        widthm1: u8,
    }
    let fn_state = FunctionState {
        Rn,
        imm3,
        Rd,
        imm2,
        widthm1,
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
        // D s_2_0: read-var Rd:u8
        let s_2_0: u8 = fn_state.Rd;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // D s_2_4: write-var d <= s_2_3
        fn_state.d = s_2_3;
        // D s_2_5: read-var Rn:u8
        let s_2_5: u8 = fn_state.Rn;
        // D s_2_6: cast zx s_2_5 -> bv
        let s_2_6: Bits = Bits::new(s_2_5 as u128, 4u16);
        // D s_2_7: cast zx s_2_6 -> i
        let s_2_7: i128 = (s_2_6.value() as i128);
        // D s_2_8: cast reint s_2_7 -> i64
        let s_2_8: i64 = (s_2_7 as i64);
        // D s_2_9: write-var n <= s_2_8
        fn_state.n = s_2_8;
        // D s_2_10: read-var imm3:u8
        let s_2_10: u8 = fn_state.imm3;
        // D s_2_11: cast zx s_2_10 -> bv
        let s_2_11: Bits = Bits::new(s_2_10 as u128, 3u16);
        // D s_2_12: read-var imm2:u8
        let s_2_12: u8 = fn_state.imm2;
        // D s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 2u16);
        // D s_2_14: cast reint s_2_11 -> u128
        let s_2_14: u128 = (s_2_11.value() as u128);
        // D s_2_15: size-of s_2_11
        let s_2_15: u16 = s_2_11.length();
        // D s_2_16: cast reint s_2_13 -> u128
        let s_2_16: u128 = (s_2_13.value() as u128);
        // D s_2_17: size-of s_2_13
        let s_2_17: u16 = s_2_13.length();
        // D s_2_18: lsl s_2_14 s_2_17
        let s_2_18: u128 = s_2_14 << s_2_17;
        // D s_2_19: or s_2_18 s_2_16
        let s_2_19: u128 = ((s_2_18) | (s_2_16));
        // D s_2_20: add s_2_15 s_2_17
        let s_2_20: u16 = (s_2_15 + s_2_17);
        // D s_2_21: create-bits s_2_19 s_2_20
        let s_2_21: Bits = Bits::new(s_2_19, s_2_20);
        // D s_2_22: cast reint s_2_21 -> u8
        let s_2_22: u8 = (s_2_21.value() as u8);
        // D s_2_23: cast zx s_2_22 -> bv
        let s_2_23: Bits = Bits::new(s_2_22 as u128, 5u16);
        // D s_2_24: cast zx s_2_23 -> i
        let s_2_24: i128 = (s_2_23.value() as i128);
        // D s_2_25: cast reint s_2_24 -> i64
        let s_2_25: i64 = (s_2_24 as i64);
        // D s_2_26: write-var lsbit <= s_2_25
        fn_state.lsbit = s_2_25;
        // D s_2_27: read-var widthm1:u8
        let s_2_27: u8 = fn_state.widthm1;
        // D s_2_28: cast zx s_2_27 -> bv
        let s_2_28: Bits = Bits::new(s_2_27 as u128, 5u16);
        // D s_2_29: cast zx s_2_28 -> i
        let s_2_29: i128 = (s_2_28.value() as i128);
        // D s_2_30: cast reint s_2_29 -> i64
        let s_2_30: i64 = (s_2_29 as i64);
        // D s_2_31: read-var lsbit:i64
        let s_2_31: i64 = fn_state.lsbit;
        // D s_2_32: cast zx s_2_31 -> i
        let s_2_32: i128 = (i128::try_from(s_2_31).unwrap());
        // D s_2_33: cast zx s_2_30 -> i
        let s_2_33: i128 = (i128::try_from(s_2_30).unwrap());
        // D s_2_34: add s_2_32 s_2_33
        let s_2_34: i128 = (s_2_32 + s_2_33);
        // D s_2_35: cast reint s_2_34 -> i64
        let s_2_35: i64 = (s_2_34 as i64);
        // D s_2_36: write-var msbit <= s_2_35
        fn_state.msbit = s_2_35;
        // C s_2_37: const #15s : i
        let s_2_37: i128 = 15;
        // D s_2_38: read-var d:i64
        let s_2_38: i64 = fn_state.d;
        // D s_2_39: cast zx s_2_38 -> i
        let s_2_39: i128 = (i128::try_from(s_2_38).unwrap());
        // D s_2_40: cmp-eq s_2_39 s_2_37
        let s_2_40: bool = ((s_2_39) == (s_2_37));
        // N s_2_41: branch s_2_40 b9 b3
        if s_2_40 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #15s : i
        let s_3_0: i128 = 15;
        // D s_3_1: read-var n:i64
        let s_3_1: i64 = fn_state.n;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: cmp-eq s_3_2 s_3_0
        let s_3_3: bool = ((s_3_2) == (s_3_0));
        // D s_3_4: write-var gs#300592 <= s_3_3
        fn_state.gs_300592 = s_3_3;
        // N s_3_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#300592:u8
        let s_4_0: bool = fn_state.gs_300592;
        // N s_4_1: branch s_4_0 b8 b5
        if s_4_0 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #31s : i
        let s_5_0: i128 = 31;
        // D s_5_1: read-var msbit:i64
        let s_5_1: i64 = fn_state.msbit;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: cmp-gt s_5_2 s_5_0
        let s_5_3: bool = ((s_5_2) > (s_5_0));
        // N s_5_4: branch s_5_3 b7 b6
        if s_5_3 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var msbit:i64
        let s_6_0: i64 = fn_state.msbit;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: read-var d:i64
        let s_6_2: i64 = fn_state.d;
        // D s_6_3: read-var lsbit:i64
        let s_6_3: i64 = fn_state.lsbit;
        // D s_6_4: read-var n:i64
        let s_6_4: i64 = fn_state.n;
        // D s_6_5: call execute_aarch32_instrs_SBFX_Op_A_txt(s_6_2, s_6_3, s_6_1, s_6_4)
        let s_6_5: () = execute_aarch32_instrs_SBFX_Op_A_txt(
            state,
            tracer,
            s_6_2,
            s_6_3,
            s_6_1,
            s_6_4,
        );
        // N s_6_6: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: panic
        panic!("{:?}", ());
        // N s_7_1: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: panic
        panic!("{:?}", ());
        // N s_8_1: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var gs#300592 <= s_9_0
        fn_state.gs_300592 = s_9_0;
        // N s_9_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
