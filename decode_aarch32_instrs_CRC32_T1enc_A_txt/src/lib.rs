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
use execute_aarch32_instrs_CRC32_Op_A_txt::*;
use ConditionPassed::*;
use HaveCRCExt::*;
use InITBlock::*;
use common::*;
pub fn decode_aarch32_instrs_CRC32_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    C: bool,
    Rn: u8,
    Rd: u8,
    sz: u8,
    Rm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        gs_323916: bool,
        crc32c: bool,
        gs_323918: bool,
        n: i64,
        d: i64,
        size: i64,
        C: bool,
        Rn: u8,
        Rd: u8,
        sz: u8,
        Rm: u8,
    }
    let fn_state = FunctionState {
        C,
        Rn,
        Rd,
        sz,
        Rm,
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
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call InITBlock(s_2_0)
        let s_2_1: bool = InITBlock(state, tracer, s_2_0);
        // N s_2_2: branch s_2_1 b16 b3
        if s_2_1 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call HaveCRCExt(s_3_0)
        let s_3_1: bool = HaveCRCExt(state, tracer, s_3_0);
        // S s_3_2: not s_3_1
        let s_3_2: bool = !s_3_1;
        // N s_3_3: branch s_3_2 b15 b4
        if s_3_2 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var Rd:u8
        let s_4_0: u8 = fn_state.Rd;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 4u16);
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (s_4_1.value() as i128);
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // D s_4_4: write-var d <= s_4_3
        fn_state.d = s_4_3;
        // D s_4_5: read-var Rn:u8
        let s_4_5: u8 = fn_state.Rn;
        // D s_4_6: cast zx s_4_5 -> bv
        let s_4_6: Bits = Bits::new(s_4_5 as u128, 4u16);
        // D s_4_7: cast zx s_4_6 -> i
        let s_4_7: i128 = (s_4_6.value() as i128);
        // D s_4_8: cast reint s_4_7 -> i64
        let s_4_8: i64 = (s_4_7 as i64);
        // D s_4_9: write-var n <= s_4_8
        fn_state.n = s_4_8;
        // D s_4_10: read-var Rm:u8
        let s_4_10: u8 = fn_state.Rm;
        // D s_4_11: cast zx s_4_10 -> bv
        let s_4_11: Bits = Bits::new(s_4_10 as u128, 4u16);
        // D s_4_12: cast zx s_4_11 -> i
        let s_4_12: i128 = (s_4_11.value() as i128);
        // D s_4_13: cast reint s_4_12 -> i64
        let s_4_13: i64 = (s_4_12 as i64);
        // D s_4_14: write-var m <= s_4_13
        fn_state.m = s_4_13;
        // D s_4_15: read-var sz:u8
        let s_4_15: u8 = fn_state.sz;
        // D s_4_16: cast zx s_4_15 -> bv
        let s_4_16: Bits = Bits::new(s_4_15 as u128, 2u16);
        // D s_4_17: cast zx s_4_16 -> i
        let s_4_17: i128 = (s_4_16.value() as i128);
        // D s_4_18: cast reint s_4_17 -> i64
        let s_4_18: i64 = (s_4_17 as i64);
        // C s_4_19: const #8s : i64
        let s_4_19: i64 = 8;
        // D s_4_20: lsl s_4_19 s_4_18
        let s_4_20: i64 = s_4_19 << s_4_18;
        // D s_4_21: write-var size <= s_4_20
        fn_state.size = s_4_20;
        // D s_4_22: read-var C:u8
        let s_4_22: bool = fn_state.C;
        // D s_4_23: cast zx s_4_22 -> bv
        let s_4_23: Bits = Bits::new(s_4_22 as u128, 1u16);
        // C s_4_24: const #1u : u8
        let s_4_24: bool = true;
        // C s_4_25: cast zx s_4_24 -> bv
        let s_4_25: Bits = Bits::new(s_4_24 as u128, 1u16);
        // D s_4_26: cmp-eq s_4_23 s_4_25
        let s_4_26: bool = ((s_4_23) == (s_4_25));
        // D s_4_27: write-var crc32c <= s_4_26
        fn_state.crc32c = s_4_26;
        // C s_4_28: const #15s : i
        let s_4_28: i128 = 15;
        // D s_4_29: read-var d:i64
        let s_4_29: i64 = fn_state.d;
        // D s_4_30: cast zx s_4_29 -> i
        let s_4_30: i128 = (i128::try_from(s_4_29).unwrap());
        // D s_4_31: cmp-eq s_4_30 s_4_28
        let s_4_31: bool = ((s_4_30) == (s_4_28));
        // N s_4_32: branch s_4_31 b14 b5
        if s_4_31 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #15s : i
        let s_5_0: i128 = 15;
        // D s_5_1: read-var n:i64
        let s_5_1: i64 = fn_state.n;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: cmp-eq s_5_2 s_5_0
        let s_5_3: bool = ((s_5_2) == (s_5_0));
        // D s_5_4: write-var gs#323916 <= s_5_3
        fn_state.gs_323916 = s_5_3;
        // N s_5_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#323916:u8
        let s_6_0: bool = fn_state.gs_323916;
        // N s_6_1: branch s_6_0 b13 b7
        if s_6_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #15s : i
        let s_7_0: i128 = 15;
        // D s_7_1: read-var m:i64
        let s_7_1: i64 = fn_state.m;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: cmp-eq s_7_2 s_7_0
        let s_7_3: bool = ((s_7_2) == (s_7_0));
        // D s_7_4: write-var gs#323918 <= s_7_3
        fn_state.gs_323918 = s_7_3;
        // N s_7_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#323918:u8
        let s_8_0: bool = fn_state.gs_323918;
        // N s_8_1: branch s_8_0 b12 b9
        if s_8_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #64s : i
        let s_9_0: i128 = 64;
        // D s_9_1: read-var size:i64
        let s_9_1: i64 = fn_state.size;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: cmp-eq s_9_2 s_9_0
        let s_9_3: bool = ((s_9_2) == (s_9_0));
        // N s_9_4: branch s_9_3 b11 b10
        if s_9_3 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var size:i64
        let s_10_0: i64 = fn_state.size;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // D s_10_3: read-var crc32c:u8
        let s_10_3: bool = fn_state.crc32c;
        // D s_10_4: read-var d:i64
        let s_10_4: i64 = fn_state.d;
        // D s_10_5: read-var m:i64
        let s_10_5: i64 = fn_state.m;
        // D s_10_6: read-var n:i64
        let s_10_6: i64 = fn_state.n;
        // D s_10_7: call execute_aarch32_instrs_CRC32_Op_A_txt(s_10_3, s_10_4, s_10_5, s_10_6, s_10_2)
        let s_10_7: () = execute_aarch32_instrs_CRC32_Op_A_txt(
            state,
            tracer,
            s_10_3,
            s_10_4,
            s_10_5,
            s_10_6,
            s_10_2,
        );
        // N s_10_8: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: panic
        panic!("{:?}", ());
        // N s_11_1: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: panic
        panic!("{:?}", ());
        // N s_12_1: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#323918 <= s_13_0
        fn_state.gs_323918 = s_13_0;
        // N s_13_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#323916 <= s_14_0
        fn_state.gs_323916 = s_14_0;
        // N s_14_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: panic
        panic!("{:?}", ());
        // N s_15_1: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: panic
        panic!("{:?}", ());
        // N s_16_1: return
        return;
    }
}
