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
use neq_int::*;
use HavePACExt::*;
use execute_aarch64_instrs_branch_unconditional_register::*;
use common::*;
pub fn decode_bra_aarch64_instrs_branch_unconditional_register<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rm: u8,
    Rn: u8,
    M: bool,
    A: bool,
    op: u8,
    Z: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        gs_144767: bool,
        gs_144754: bool,
        branch_typeshadow_1104: u32,
        n: i64,
        gs_144764: bool,
        branch_type: u32,
        gs_144756: bool,
        use_key_a: bool,
        pac: bool,
        source_is_sp: bool,
        Rm: u8,
        Rn: u8,
        M: bool,
        A: bool,
        op: u8,
        Z: bool,
    }
    let fn_state = FunctionState {
        Rm,
        Rn,
        M,
        A,
        op,
        Z,
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
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 5u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: write-var n <= s_0_3
        fn_state.n = s_0_3;
        // D s_0_5: read-var Rm:u8
        let s_0_5: u8 = fn_state.Rm;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 5u16);
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (s_0_6.value() as i128);
        // D s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // D s_0_9: write-var m <= s_0_8
        fn_state.m = s_0_8;
        // D s_0_10: read-var A:u8
        let s_0_10: bool = fn_state.A;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 1u16);
        // C s_0_12: const #1u : u8
        let s_0_12: bool = true;
        // C s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 1u16);
        // D s_0_14: cmp-eq s_0_11 s_0_13
        let s_0_14: bool = ((s_0_11) == (s_0_13));
        // D s_0_15: write-var pac <= s_0_14
        fn_state.pac = s_0_14;
        // D s_0_16: read-var M:u8
        let s_0_16: bool = fn_state.M;
        // D s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 1u16);
        // C s_0_18: const #0u : u8
        let s_0_18: bool = false;
        // C s_0_19: cast zx s_0_18 -> bv
        let s_0_19: Bits = Bits::new(s_0_18 as u128, 1u16);
        // D s_0_20: cmp-eq s_0_17 s_0_19
        let s_0_20: bool = ((s_0_17) == (s_0_19));
        // D s_0_21: write-var use_key_a <= s_0_20
        fn_state.use_key_a = s_0_20;
        // D s_0_22: read-var Z:u8
        let s_0_22: bool = fn_state.Z;
        // D s_0_23: cast zx s_0_22 -> bv
        let s_0_23: Bits = Bits::new(s_0_22 as u128, 1u16);
        // C s_0_24: const #1u : u8
        let s_0_24: bool = true;
        // C s_0_25: cast zx s_0_24 -> bv
        let s_0_25: Bits = Bits::new(s_0_24 as u128, 1u16);
        // D s_0_26: cmp-eq s_0_23 s_0_25
        let s_0_26: bool = ((s_0_23) == (s_0_25));
        // N s_0_27: branch s_0_26 b33 b1
        if s_0_26 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#144754 <= s_1_0
        fn_state.gs_144754 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#144754:u8
        let s_2_0: bool = fn_state.gs_144754;
        // D s_2_1: write-var source_is_sp <= s_2_0
        fn_state.source_is_sp = s_2_0;
        // D s_2_2: read-var pac:u8
        let s_2_2: bool = fn_state.pac;
        // D s_2_3: not s_2_2
        let s_2_3: bool = !s_2_2;
        // N s_2_4: branch s_2_3 b32 b3
        if s_2_3 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#144756 <= s_3_0
        fn_state.gs_144756 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#144756:u8
        let s_4_0: bool = fn_state.gs_144756;
        // N s_4_1: branch s_4_0 b31 b5
        if s_4_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var pac:u8
        let s_5_0: bool = fn_state.pac;
        // N s_5_1: branch s_5_0 b30 b6
        if s_5_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#144764 <= s_6_0
        fn_state.gs_144764 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#144764:u8
        let s_7_0: bool = fn_state.gs_144764;
        // N s_7_1: branch s_7_0 b29 b8
        if s_7_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var op:u8
        let s_8_0: u8 = fn_state.op;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 2u16);
        // C s_8_2: const #0u : u8
        let s_8_2: u8 = 0;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 2u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // D s_8_5: not s_8_4
        let s_8_5: bool = !s_8_4;
        // N s_8_6: branch s_8_5 b24 b9
        if s_8_5 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #6u : u32
        let s_9_0: u32 = 6;
        // D s_9_1: write-var branch_type <= s_9_0
        fn_state.branch_type = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var branch_type:u32
        let s_10_0: u32 = fn_state.branch_type;
        // D s_10_1: write-var branch_typeshadow#1104 <= s_10_0
        fn_state.branch_typeshadow_1104 = s_10_0;
        // D s_10_2: read-var pac:u8
        let s_10_2: bool = fn_state.pac;
        // N s_10_3: branch s_10_2 b13 b11
        if s_10_2 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var n:i64
        let s_12_0: i64 = fn_state.n;
        // D s_12_1: read-var branch_typeshadow#1104:u32
        let s_12_1: u32 = fn_state.branch_typeshadow_1104;
        // D s_12_2: read-var m:i64
        let s_12_2: i64 = fn_state.m;
        // D s_12_3: read-var pac:u8
        let s_12_3: bool = fn_state.pac;
        // D s_12_4: read-var source_is_sp:u8
        let s_12_4: bool = fn_state.source_is_sp;
        // D s_12_5: read-var use_key_a:u8
        let s_12_5: bool = fn_state.use_key_a;
        // D s_12_6: call execute_aarch64_instrs_branch_unconditional_register(s_12_1, s_12_2, s_12_0, s_12_3, s_12_4, s_12_5)
        let s_12_6: () = execute_aarch64_instrs_branch_unconditional_register(
            state,
            tracer,
            s_12_1,
            s_12_2,
            s_12_0,
            s_12_3,
            s_12_4,
            s_12_5,
        );
        // N s_12_7: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var Z:u8
        let s_13_0: bool = fn_state.Z;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 1u16);
        // C s_13_2: const #0u : u8
        let s_13_2: bool = false;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 1u16);
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // N s_13_5: branch s_13_4 b23 b14
        if s_13_4 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#144767 <= s_14_0
        fn_state.gs_144767 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#144767:u8
        let s_15_0: bool = fn_state.gs_144767;
        // N s_15_1: branch s_15_0 b22 b16
        if s_15_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var branch_typeshadow#1104:u32
        let s_16_0: u32 = fn_state.branch_typeshadow_1104;
        // C s_16_1: const #4u : u32
        let s_16_1: u32 = 4;
        // D s_16_2: cmp-eq s_16_0 s_16_1
        let s_16_2: bool = ((s_16_0) == (s_16_1));
        // N s_16_3: branch s_16_2 b19 b17
        if s_16_2 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #31s : i
        let s_19_0: i128 = 31;
        // D s_19_1: read-var n:i64
        let s_19_1: i64 = fn_state.n;
        // D s_19_2: cast zx s_19_1 -> i
        let s_19_2: i128 = (i128::try_from(s_19_1).unwrap());
        // D s_19_3: call neq_int(s_19_2, s_19_0)
        let s_19_3: bool = neq_int(state, tracer, s_19_2, s_19_0);
        // N s_19_4: branch s_19_3 b21 b20
        if s_19_3 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #30s : i64
        let s_20_0: i64 = 30;
        // D s_20_1: write-var n <= s_20_0
        fn_state.n = s_20_0;
        // C s_20_2: const #1u : u8
        let s_20_2: bool = true;
        // D s_20_3: write-var source_is_sp <= s_20_2
        fn_state.source_is_sp = s_20_2;
        // N s_20_4: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_21_0: panic
        panic!("{:?}", ());
        // N s_21_1: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: panic
        panic!("{:?}", ());
        // N s_22_1: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #31s : i
        let s_23_0: i128 = 31;
        // D s_23_1: read-var m:i64
        let s_23_1: i64 = fn_state.m;
        // D s_23_2: cast zx s_23_1 -> i
        let s_23_2: i128 = (i128::try_from(s_23_1).unwrap());
        // D s_23_3: call neq_int(s_23_2, s_23_0)
        let s_23_3: bool = neq_int(state, tracer, s_23_2, s_23_0);
        // D s_23_4: write-var gs#144767 <= s_23_3
        fn_state.gs_144767 = s_23_3;
        // N s_23_5: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var op:u8
        let s_24_0: u8 = fn_state.op;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 2u16);
        // C s_24_2: const #1u : u8
        let s_24_2: u8 = 1;
        // C s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 2u16);
        // D s_24_4: cmp-eq s_24_1 s_24_3
        let s_24_4: bool = ((s_24_1) == (s_24_3));
        // D s_24_5: not s_24_4
        let s_24_5: bool = !s_24_4;
        // N s_24_6: branch s_24_5 b26 b25
        if s_24_5 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #1u : u32
        let s_25_0: u32 = 1;
        // D s_25_1: write-var branch_type <= s_25_0
        fn_state.branch_type = s_25_0;
        // N s_25_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var op:u8
        let s_26_0: u8 = fn_state.op;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 2u16);
        // C s_26_2: const #2u : u8
        let s_26_2: u8 = 2;
        // C s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 2u16);
        // D s_26_4: cmp-eq s_26_1 s_26_3
        let s_26_4: bool = ((s_26_1) == (s_26_3));
        // D s_26_5: not s_26_4
        let s_26_5: bool = !s_26_4;
        // N s_26_6: branch s_26_5 b28 b27
        if s_26_5 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #4u : u32
        let s_27_0: u32 = 4;
        // D s_27_1: write-var branch_type <= s_27_0
        fn_state.branch_type = s_27_0;
        // N s_27_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_28_0: panic
        panic!("{:?}", ());
        // N s_28_1: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_29_0: panic
        panic!("{:?}", ());
        // N s_29_1: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #() : ()
        let s_30_0: () = ();
        // S s_30_1: call HavePACExt(s_30_0)
        let s_30_1: bool = HavePACExt(state, tracer, s_30_0);
        // S s_30_2: not s_30_1
        let s_30_2: bool = !s_30_1;
        // D s_30_3: write-var gs#144764 <= s_30_2
        fn_state.gs_144764 = s_30_2;
        // N s_30_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_31_0: panic
        panic!("{:?}", ());
        // N s_31_1: return
        return;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #0s : i
        let s_32_0: i128 = 0;
        // D s_32_1: read-var m:i64
        let s_32_1: i64 = fn_state.m;
        // D s_32_2: cast zx s_32_1 -> i
        let s_32_2: i128 = (i128::try_from(s_32_1).unwrap());
        // D s_32_3: call neq_int(s_32_2, s_32_0)
        let s_32_3: bool = neq_int(state, tracer, s_32_2, s_32_0);
        // D s_32_4: write-var gs#144756 <= s_32_3
        fn_state.gs_144756 = s_32_3;
        // N s_32_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #31s : i
        let s_33_0: i128 = 31;
        // D s_33_1: read-var m:i64
        let s_33_1: i64 = fn_state.m;
        // D s_33_2: cast zx s_33_1 -> i
        let s_33_2: i128 = (i128::try_from(s_33_1).unwrap());
        // D s_33_3: cmp-eq s_33_2 s_33_0
        let s_33_3: bool = ((s_33_2) == (s_33_0));
        // D s_33_4: write-var gs#144754 <= s_33_3
        fn_state.gs_144754 = s_33_3;
        // N s_33_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
