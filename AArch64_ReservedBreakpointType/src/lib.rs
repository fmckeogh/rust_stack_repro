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
use u__UNKNOWN_bits::*;
use HaveAArch32EL::*;
use NumContextAwareBreakpointsImplemented::*;
use u__UNKNOWN_bit::*;
use ConstrainUnpredictableBits::*;
use HaveFeatABLE::*;
use HaveV82Debug::*;
use HaveVirtHostExt::*;
use NumBreakpointsImplemented::*;
use common::*;
pub fn AArch64_ReservedBreakpointType<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i128,
    bt2_in: bool,
    bt_in: u8,
) -> ProductTypeea264718a40d3f4a {
    #[derive(Default)]
    struct FunctionState {
        gs_15956: bool,
        gs_15931: bool,
        gs_15962: bool,
        gs_15972: bool,
        gs_15970: bool,
        gs_15943: bool,
        reserved: bool,
        bt2: bool,
        b__0: u8,
        gs_15955: bool,
        gs_444222: ProductType690b94b58c91cec7,
        gs_15938: bool,
        gs_15971: bool,
        b__1: u8,
        c: u32,
        gs_15947: bool,
        gs_15952: bool,
        return_value: ProductTypeea264718a40d3f4a,
        gs_15977: bool,
        gs_15978: bool,
        context_aware: bool,
        bt: u8,
        gs_15961: bool,
        n: i128,
        bt2_in: bool,
        bt_in: u8,
    }
    let fn_state = FunctionState {
        n,
        bt2_in,
        bt_in,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // D s_0_0: read-var bt2_in:u8
        let s_0_0: bool = fn_state.bt2_in;
        // D s_0_1: write-var bt2 <= s_0_0
        fn_state.bt2 = s_0_0;
        // D s_0_2: read-var bt_in:u8
        let s_0_2: u8 = fn_state.bt_in;
        // D s_0_3: write-var bt <= s_0_2
        fn_state.bt = s_0_2;
        // C s_0_4: const #0u : u8
        let s_0_4: bool = false;
        // D s_0_5: write-var reserved <= s_0_4
        fn_state.reserved = s_0_4;
        // C s_0_6: const #() : ()
        let s_0_6: () = ();
        // S s_0_7: call NumBreakpointsImplemented(s_0_6)
        let s_0_7: i128 = NumBreakpointsImplemented(state, tracer, s_0_6);
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call NumContextAwareBreakpointsImplemented(s_0_8)
        let s_0_9: i128 = NumContextAwareBreakpointsImplemented(state, tracer, s_0_8);
        // S s_0_10: sub s_0_7 s_0_9
        let s_0_10: i128 = ((s_0_7) - (s_0_9));
        // D s_0_11: read-var n:i
        let s_0_11: i128 = fn_state.n;
        // D s_0_12: cmp-ge s_0_11 s_0_10
        let s_0_12: bool = ((s_0_11) >= (s_0_10));
        // D s_0_13: write-var context_aware <= s_0_12
        fn_state.context_aware = s_0_12;
        // D s_0_14: read-var bt2:u8
        let s_0_14: bool = fn_state.bt2;
        // D s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 1u16);
        // C s_0_16: const #0u : u8
        let s_0_16: bool = false;
        // C s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 1u16);
        // D s_0_18: cmp-eq s_0_15 s_0_17
        let s_0_18: bool = ((s_0_15) == (s_0_17));
        // N s_0_19: branch s_0_18 b21 b1
        if s_0_18 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // D s_1_0: read-var bt:u8
        let s_1_0: u8 = fn_state.bt;
        // D s_1_1: write-var b__0 <= s_1_0
        fn_state.b__0 = s_1_0;
        // C s_1_2: const #3s : i
        let s_1_2: i128 = 3;
        // D s_1_3: read-var b__0:u8
        let s_1_3: u8 = fn_state.b__0;
        // D s_1_4: cast zx s_1_3 -> bv
        let s_1_4: Bits = Bits::new(s_1_3 as u128, 4u16);
        // C s_1_5: const #1s : i64
        let s_1_5: i64 = 1;
        // C s_1_6: cast zx s_1_5 -> i
        let s_1_6: i128 = (i128::try_from(s_1_5).unwrap());
        // C s_1_7: const #0s : i
        let s_1_7: i128 = 0;
        // C s_1_8: add s_1_7 s_1_6
        let s_1_8: i128 = (s_1_7 + s_1_6);
        // D s_1_9: bit-extract s_1_4 s_1_2 s_1_8
        let s_1_9: Bits = (Bits::new(
            ((s_1_4) >> (s_1_2)).value(),
            u16::try_from(s_1_8).unwrap(),
        ));
        // D s_1_10: cast reint s_1_9 -> u8
        let s_1_10: bool = ((s_1_9.value()) != 0);
        // D s_1_11: cast zx s_1_10 -> bv
        let s_1_11: Bits = Bits::new(s_1_10 as u128, 1u16);
        // C s_1_12: const #0u : u8
        let s_1_12: bool = false;
        // C s_1_13: cast zx s_1_12 -> bv
        let s_1_13: Bits = Bits::new(s_1_12 as u128, 1u16);
        // D s_1_14: cmp-eq s_1_11 s_1_13
        let s_1_14: bool = ((s_1_11) == (s_1_13));
        // N s_1_15: branch s_1_14 b20 b2
        if s_1_14 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#15943 <= s_2_0
        fn_state.gs_15943 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // D s_3_0: read-var gs#15943:u8
        let s_3_0: bool = fn_state.gs_15943;
        // D s_3_1: not s_3_0
        let s_3_1: bool = !s_3_0;
        // N s_3_2: branch s_3_1 b19 b4
        if s_3_1 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_4_0: const #1u : u8
        let s_4_0: bool = true;
        // D s_4_1: write-var gs#15938 <= s_4_0
        fn_state.gs_15938 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // D s_5_0: read-var gs#15938:u8
        let s_5_0: bool = fn_state.gs_15938;
        // D s_5_1: not s_5_0
        let s_5_1: bool = !s_5_0;
        // N s_5_2: branch s_5_1 b18 b6
        if s_5_1 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // N s_6_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // D s_8_0: read-var reserved:u8
        let s_8_0: bool = fn_state.reserved;
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
    ) -> ProductTypeea264718a40d3f4a {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_10_0: const #0u : u32
        let s_10_0: u32 = 0;
        // D s_10_1: read-var bt2:u8
        let s_10_1: bool = fn_state.bt2;
        // D s_10_2: read-var bt:u8
        let s_10_2: u8 = fn_state.bt;
        // D s_10_3: create-product struct = ["s_10_0", "s_10_1", "s_10_2"]
        let s_10_3: ProductTypeea264718a40d3f4a = ProductTypeea264718a40d3f4a {
            _0: s_10_0,
            _1: s_10_1,
            _2: s_10_2,
        };
        // D s_10_4: write-var return_value <= s_10_3
        fn_state.return_value = s_10_3;
        // N s_10_5: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // D s_11_0: read-var return_value:struct
        let s_11_0: ProductTypeea264718a40d3f4a = fn_state.return_value;
        // N s_11_1: return s_11_0
        return s_11_0;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_12_0: const #5s : i
        let s_12_0: i128 = 5;
        // C s_12_1: const #33u : u32
        let s_12_1: u32 = 33;
        // S s_12_2: call ConstrainUnpredictableBits(s_12_1, s_12_0)
        let s_12_2: ProductType690b94b58c91cec7 = ConstrainUnpredictableBits(
            state,
            tracer,
            s_12_1,
            s_12_0,
        );
        // D s_12_3: write-var gs#444222 <= s_12_2
        fn_state.gs_444222 = s_12_2;
        // D s_12_4: read-var gs#444222.0:struct
        let s_12_4: u32 = fn_state.gs_444222._0;
        // D s_12_5: read-var gs#444222.1:struct
        let s_12_5: Bits = fn_state.gs_444222._1;
        // D s_12_6: cast reint s_12_5 -> u8
        let s_12_6: u8 = (s_12_5.value() as u8);
        // D s_12_7: write-var c <= s_12_4
        fn_state.c = s_12_4;
        // C s_12_8: const #4s : i
        let s_12_8: i128 = 4;
        // D s_12_9: cast zx s_12_6 -> bv
        let s_12_9: Bits = Bits::new(s_12_6 as u128, 5u16);
        // C s_12_10: const #1s : i64
        let s_12_10: i64 = 1;
        // C s_12_11: cast zx s_12_10 -> i
        let s_12_11: i128 = (i128::try_from(s_12_10).unwrap());
        // C s_12_12: const #0s : i
        let s_12_12: i128 = 0;
        // C s_12_13: add s_12_12 s_12_11
        let s_12_13: i128 = (s_12_12 + s_12_11);
        // D s_12_14: bit-extract s_12_9 s_12_8 s_12_13
        let s_12_14: Bits = (Bits::new(
            ((s_12_9) >> (s_12_8)).value(),
            u16::try_from(s_12_13).unwrap(),
        ));
        // D s_12_15: cast reint s_12_14 -> u8
        let s_12_15: bool = ((s_12_14.value()) != 0);
        // D s_12_16: write-var bt2 <= s_12_15
        fn_state.bt2 = s_12_15;
        // C s_12_17: const #0s : i
        let s_12_17: i128 = 0;
        // D s_12_18: cast zx s_12_6 -> bv
        let s_12_18: Bits = Bits::new(s_12_6 as u128, 5u16);
        // C s_12_19: const #1s : i64
        let s_12_19: i64 = 1;
        // C s_12_20: cast zx s_12_19 -> i
        let s_12_20: i128 = (i128::try_from(s_12_19).unwrap());
        // C s_12_21: const #3s : i
        let s_12_21: i128 = 3;
        // C s_12_22: add s_12_21 s_12_20
        let s_12_22: i128 = (s_12_21 + s_12_20);
        // D s_12_23: bit-extract s_12_18 s_12_17 s_12_22
        let s_12_23: Bits = (Bits::new(
            ((s_12_18) >> (s_12_17)).value(),
            u16::try_from(s_12_22).unwrap(),
        ));
        // D s_12_24: cast reint s_12_23 -> u8
        let s_12_24: u8 = (s_12_23.value() as u8);
        // D s_12_25: write-var bt <= s_12_24
        fn_state.bt = s_12_24;
        // D s_12_26: read-var c:u32
        let s_12_26: u32 = fn_state.c;
        // C s_12_27: const #7u : u32
        let s_12_27: u32 = 7;
        // D s_12_28: cmp-eq s_12_26 s_12_27
        let s_12_28: bool = ((s_12_26) == (s_12_27));
        // N s_12_29: branch s_12_28 b17 b13
        if s_12_28 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // D s_13_0: read-var c:u32
        let s_13_0: u32 = fn_state.c;
        // C s_13_1: const #1u : u32
        let s_13_1: u32 = 1;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // D s_13_3: write-var gs#15931 <= s_13_2
        fn_state.gs_15931 = s_13_2;
        // N s_13_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // D s_14_0: read-var gs#15931:u8
        let s_14_0: bool = fn_state.gs_15931;
        // N s_14_1: assert s_14_0
        let s_14_1: () = assert!(s_14_0);
        // D s_14_2: read-var c:u32
        let s_14_2: u32 = fn_state.c;
        // C s_14_3: const #7u : u32
        let s_14_3: u32 = 7;
        // D s_14_4: cmp-eq s_14_2 s_14_3
        let s_14_4: bool = ((s_14_2) == (s_14_3));
        // N s_14_5: branch s_14_4 b16 b15
        if s_14_4 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // N s_15_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call __UNKNOWN_bit(s_16_0)
        let s_16_1: bool = u__UNKNOWN_bit(state, tracer, s_16_0);
        // C s_16_2: const #4s : i64
        let s_16_2: i64 = 4;
        // C s_16_3: cast zx s_16_2 -> i
        let s_16_3: i128 = (i128::try_from(s_16_2).unwrap());
        // S s_16_4: call __UNKNOWN_bits(s_16_3)
        let s_16_4: Bits = u__UNKNOWN_bits(state, tracer, s_16_3);
        // S s_16_5: cast reint s_16_4 -> u8
        let s_16_5: u8 = (s_16_4.value() as u8);
        // D s_16_6: read-var c:u32
        let s_16_6: u32 = fn_state.c;
        // D s_16_7: create-product struct = ["s_16_6", "s_16_1", "s_16_5"]
        let s_16_7: ProductTypeea264718a40d3f4a = ProductTypeea264718a40d3f4a {
            _0: s_16_6,
            _1: s_16_1,
            _2: s_16_5,
        };
        // D s_16_8: write-var return_value <= s_16_7
        fn_state.return_value = s_16_7;
        // N s_16_9: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#15931 <= s_17_0
        fn_state.gs_15931 = s_17_0;
        // N s_17_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var reserved <= s_18_0
        fn_state.reserved = s_18_0;
        // N s_18_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#15938 <= s_19_0
        fn_state.gs_15938 = s_19_0;
        // N s_19_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_20_0: const #1s : i
        let s_20_0: i128 = 1;
        // D s_20_1: read-var b__0:u8
        let s_20_1: u8 = fn_state.b__0;
        // D s_20_2: cast zx s_20_1 -> bv
        let s_20_2: Bits = Bits::new(s_20_1 as u128, 4u16);
        // C s_20_3: const #1s : i64
        let s_20_3: i64 = 1;
        // C s_20_4: cast zx s_20_3 -> i
        let s_20_4: i128 = (i128::try_from(s_20_3).unwrap());
        // C s_20_5: const #0s : i
        let s_20_5: i128 = 0;
        // C s_20_6: add s_20_5 s_20_4
        let s_20_6: i128 = (s_20_5 + s_20_4);
        // D s_20_7: bit-extract s_20_2 s_20_0 s_20_6
        let s_20_7: Bits = (Bits::new(
            ((s_20_2) >> (s_20_0)).value(),
            u16::try_from(s_20_6).unwrap(),
        ));
        // D s_20_8: cast reint s_20_7 -> u8
        let s_20_8: bool = ((s_20_7.value()) != 0);
        // D s_20_9: cast zx s_20_8 -> bv
        let s_20_9: Bits = Bits::new(s_20_8 as u128, 1u16);
        // C s_20_10: const #0u : u8
        let s_20_10: bool = false;
        // C s_20_11: cast zx s_20_10 -> bv
        let s_20_11: Bits = Bits::new(s_20_10 as u128, 1u16);
        // D s_20_12: cmp-eq s_20_9 s_20_11
        let s_20_12: bool = ((s_20_9) == (s_20_11));
        // D s_20_13: write-var gs#15943 <= s_20_12
        fn_state.gs_15943 = s_20_12;
        // N s_20_14: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // D s_21_0: read-var bt:u8
        let s_21_0: u8 = fn_state.bt;
        // D s_21_1: write-var b__1 <= s_21_0
        fn_state.b__1 = s_21_0;
        // C s_21_2: const #3s : i
        let s_21_2: i128 = 3;
        // D s_21_3: read-var b__1:u8
        let s_21_3: u8 = fn_state.b__1;
        // D s_21_4: cast zx s_21_3 -> bv
        let s_21_4: Bits = Bits::new(s_21_3 as u128, 4u16);
        // C s_21_5: const #1s : i64
        let s_21_5: i64 = 1;
        // C s_21_6: cast zx s_21_5 -> i
        let s_21_6: i128 = (i128::try_from(s_21_5).unwrap());
        // C s_21_7: const #0s : i
        let s_21_7: i128 = 0;
        // C s_21_8: add s_21_7 s_21_6
        let s_21_8: i128 = (s_21_7 + s_21_6);
        // D s_21_9: bit-extract s_21_4 s_21_2 s_21_8
        let s_21_9: Bits = (Bits::new(
            ((s_21_4) >> (s_21_2)).value(),
            u16::try_from(s_21_8).unwrap(),
        ));
        // D s_21_10: cast reint s_21_9 -> u8
        let s_21_10: bool = ((s_21_9.value()) != 0);
        // D s_21_11: cast zx s_21_10 -> bv
        let s_21_11: Bits = Bits::new(s_21_10 as u128, 1u16);
        // C s_21_12: const #0u : u8
        let s_21_12: bool = false;
        // C s_21_13: cast zx s_21_12 -> bv
        let s_21_13: Bits = Bits::new(s_21_12 as u128, 1u16);
        // D s_21_14: cmp-eq s_21_11 s_21_13
        let s_21_14: bool = ((s_21_11) == (s_21_13));
        // N s_21_15: branch s_21_14 b68 b22
        if s_21_14 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#15952 <= s_22_0
        fn_state.gs_15952 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // D s_23_0: read-var gs#15952:u8
        let s_23_0: bool = fn_state.gs_15952;
        // D s_23_1: not s_23_0
        let s_23_1: bool = !s_23_0;
        // N s_23_2: branch s_23_1 b67 b24
        if s_23_1 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#15947 <= s_24_0
        fn_state.gs_15947 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // D s_25_0: read-var gs#15947:u8
        let s_25_0: bool = fn_state.gs_15947;
        // D s_25_1: not s_25_0
        let s_25_1: bool = !s_25_0;
        // N s_25_2: branch s_25_1 b66 b26
        if s_25_1 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var gs#15955 <= s_26_0
        fn_state.gs_15955 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // D s_27_0: read-var gs#15955:u8
        let s_27_0: bool = fn_state.gs_15955;
        // N s_27_1: branch s_27_0 b65 b28
        if s_27_0 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // N s_28_0: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // D s_29_0: read-var bt:u8
        let s_29_0: u8 = fn_state.bt;
        // C s_29_1: const #3s : i
        let s_29_1: i128 = 3;
        // D s_29_2: cast zx s_29_0 -> bv
        let s_29_2: Bits = Bits::new(s_29_0 as u128, 4u16);
        // C s_29_3: const #1s : i64
        let s_29_3: i64 = 1;
        // C s_29_4: cast zx s_29_3 -> i
        let s_29_4: i128 = (i128::try_from(s_29_3).unwrap());
        // C s_29_5: const #0s : i
        let s_29_5: i128 = 0;
        // C s_29_6: add s_29_5 s_29_4
        let s_29_6: i128 = (s_29_5 + s_29_4);
        // D s_29_7: bit-extract s_29_2 s_29_1 s_29_6
        let s_29_7: Bits = (Bits::new(
            ((s_29_2) >> (s_29_1)).value(),
            u16::try_from(s_29_6).unwrap(),
        ));
        // D s_29_8: cast reint s_29_7 -> u8
        let s_29_8: bool = ((s_29_7.value()) != 0);
        // D s_29_9: cast zx s_29_8 -> bv
        let s_29_9: Bits = Bits::new(s_29_8 as u128, 1u16);
        // C s_29_10: const #1u : u8
        let s_29_10: bool = true;
        // C s_29_11: cast zx s_29_10 -> bv
        let s_29_11: Bits = Bits::new(s_29_10 as u128, 1u16);
        // D s_29_12: cmp-eq s_29_9 s_29_11
        let s_29_12: bool = ((s_29_9) == (s_29_11));
        // D s_29_13: not s_29_12
        let s_29_13: bool = !s_29_12;
        // N s_29_14: branch s_29_13 b64 b30
        if s_29_13 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_30_0: const #1u : u8
        let s_30_0: bool = true;
        // D s_30_1: write-var gs#15956 <= s_30_0
        fn_state.gs_15956 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // D s_31_0: read-var gs#15956:u8
        let s_31_0: bool = fn_state.gs_15956;
        // N s_31_1: branch s_31_0 b63 b32
        if s_31_0 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_32_0: const #0u : u8
        let s_32_0: bool = false;
        // D s_32_1: write-var gs#15961 <= s_32_0
        fn_state.gs_15961 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // D s_33_0: read-var gs#15961:u8
        let s_33_0: bool = fn_state.gs_15961;
        // N s_33_1: branch s_33_0 b62 b34
        if s_33_0 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // N s_34_0: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // D s_35_0: read-var bt:u8
        let s_35_0: u8 = fn_state.bt;
        // C s_35_1: const #1s : i
        let s_35_1: i128 = 1;
        // D s_35_2: cast zx s_35_0 -> bv
        let s_35_2: Bits = Bits::new(s_35_0 as u128, 4u16);
        // C s_35_3: const #1s : i64
        let s_35_3: i64 = 1;
        // C s_35_4: cast zx s_35_3 -> i
        let s_35_4: i128 = (i128::try_from(s_35_3).unwrap());
        // C s_35_5: const #2s : i
        let s_35_5: i128 = 2;
        // C s_35_6: add s_35_5 s_35_4
        let s_35_6: i128 = (s_35_5 + s_35_4);
        // D s_35_7: bit-extract s_35_2 s_35_1 s_35_6
        let s_35_7: Bits = (Bits::new(
            ((s_35_2) >> (s_35_1)).value(),
            u16::try_from(s_35_6).unwrap(),
        ));
        // D s_35_8: cast reint s_35_7 -> u8
        let s_35_8: u8 = (s_35_7.value() as u8);
        // D s_35_9: cast zx s_35_8 -> bv
        let s_35_9: Bits = Bits::new(s_35_8 as u128, 3u16);
        // C s_35_10: const #3u : u8
        let s_35_10: u8 = 3;
        // C s_35_11: cast zx s_35_10 -> bv
        let s_35_11: Bits = Bits::new(s_35_10 as u128, 3u16);
        // D s_35_12: cmp-eq s_35_9 s_35_11
        let s_35_12: bool = ((s_35_9) == (s_35_11));
        // D s_35_13: not s_35_12
        let s_35_13: bool = !s_35_12;
        // N s_35_14: branch s_35_13 b59 b36
        if s_35_13 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_36_0: const #1u : u8
        let s_36_0: bool = true;
        // D s_36_1: write-var gs#15962 <= s_36_0
        fn_state.gs_15962 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // D s_37_0: read-var gs#15962:u8
        let s_37_0: bool = fn_state.gs_15962;
        // N s_37_1: branch s_37_0 b58 b38
        if s_37_0 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_38_0: const #0u : u8
        let s_38_0: bool = false;
        // D s_38_1: write-var gs#15970 <= s_38_0
        fn_state.gs_15970 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // D s_39_0: read-var gs#15970:u8
        let s_39_0: bool = fn_state.gs_15970;
        // N s_39_1: branch s_39_0 b57 b40
        if s_39_0 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_40_0: const #0u : u8
        let s_40_0: bool = false;
        // D s_40_1: write-var gs#15971 <= s_40_0
        fn_state.gs_15971 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // D s_41_0: read-var gs#15971:u8
        let s_41_0: bool = fn_state.gs_15971;
        // N s_41_1: branch s_41_0 b56 b42
        if s_41_0 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // N s_42_0: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // D s_43_0: read-var bt:u8
        let s_43_0: u8 = fn_state.bt;
        // C s_43_1: const #1s : i
        let s_43_1: i128 = 1;
        // D s_43_2: cast zx s_43_0 -> bv
        let s_43_2: Bits = Bits::new(s_43_0 as u128, 4u16);
        // C s_43_3: const #1s : i64
        let s_43_3: i64 = 1;
        // C s_43_4: cast zx s_43_3 -> i
        let s_43_4: i128 = (i128::try_from(s_43_3).unwrap());
        // C s_43_5: const #2s : i
        let s_43_5: i128 = 2;
        // C s_43_6: add s_43_5 s_43_4
        let s_43_6: i128 = (s_43_5 + s_43_4);
        // D s_43_7: bit-extract s_43_2 s_43_1 s_43_6
        let s_43_7: Bits = (Bits::new(
            ((s_43_2) >> (s_43_1)).value(),
            u16::try_from(s_43_6).unwrap(),
        ));
        // D s_43_8: cast reint s_43_7 -> u8
        let s_43_8: u8 = (s_43_7.value() as u8);
        // D s_43_9: cast zx s_43_8 -> bv
        let s_43_9: Bits = Bits::new(s_43_8 as u128, 3u16);
        // C s_43_10: const #2u : u8
        let s_43_10: u8 = 2;
        // C s_43_11: cast zx s_43_10 -> bv
        let s_43_11: Bits = Bits::new(s_43_10 as u128, 3u16);
        // D s_43_12: cmp-eq s_43_9 s_43_11
        let s_43_12: bool = ((s_43_9) == (s_43_11));
        // D s_43_13: not s_43_12
        let s_43_13: bool = !s_43_12;
        // N s_43_14: branch s_43_13 b55 b44
        if s_43_13 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_44_0: const #1u : u8
        let s_44_0: bool = true;
        // D s_44_1: write-var gs#15972 <= s_44_0
        fn_state.gs_15972 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // D s_45_0: read-var gs#15972:u8
        let s_45_0: bool = fn_state.gs_15972;
        // N s_45_1: branch s_45_0 b54 b46
        if s_45_0 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_46_0: const #0u : u8
        let s_46_0: bool = false;
        // D s_46_1: write-var gs#15977 <= s_46_0
        fn_state.gs_15977 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // D s_47_0: read-var gs#15977:u8
        let s_47_0: bool = fn_state.gs_15977;
        // N s_47_1: branch s_47_0 b53 b48
        if s_47_0 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_48_0: const #0u : u8
        let s_48_0: bool = false;
        // D s_48_1: write-var gs#15978 <= s_48_0
        fn_state.gs_15978 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // D s_49_0: read-var gs#15978:u8
        let s_49_0: bool = fn_state.gs_15978;
        // N s_49_1: branch s_49_0 b52 b50
        if s_49_0 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // N s_50_0: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // N s_51_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_52_0: const #1u : u8
        let s_52_0: bool = true;
        // D s_52_1: write-var reserved <= s_52_0
        fn_state.reserved = s_52_0;
        // N s_52_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_53_0: const #440u : u32
        let s_53_0: u32 = 440;
        // D s_53_1: read-reg s_53_0:u8
        let s_53_1: u8 = {
            let value = state.read_register::<u8>(s_53_0 as isize);
            tracer.read_register(s_53_0 as isize, value);
            value
        };
        // D s_53_2: call HaveAArch32EL(s_53_1)
        let s_53_2: bool = HaveAArch32EL(state, tracer, s_53_1);
        // D s_53_3: not s_53_2
        let s_53_3: bool = !s_53_2;
        // D s_53_4: write-var gs#15978 <= s_53_3
        fn_state.gs_15978 = s_53_3;
        // N s_53_5: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_54_0: const #() : ()
        let s_54_0: () = ();
        // S s_54_1: call HaveFeatABLE(s_54_0)
        let s_54_1: bool = HaveFeatABLE(state, tracer, s_54_0);
        // S s_54_2: not s_54_1
        let s_54_2: bool = !s_54_1;
        // D s_54_3: write-var gs#15977 <= s_54_2
        fn_state.gs_15977 = s_54_2;
        // N s_54_4: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_55_0: const #0u : u8
        let s_55_0: bool = false;
        // D s_55_1: write-var gs#15972 <= s_55_0
        fn_state.gs_15972 = s_55_0;
        // N s_55_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_56_0: const #1u : u8
        let s_56_0: bool = true;
        // D s_56_1: write-var reserved <= s_56_0
        fn_state.reserved = s_56_0;
        // N s_56_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_57_0: const #() : ()
        let s_57_0: () = ();
        // S s_57_1: call HaveV82Debug(s_57_0)
        let s_57_1: bool = HaveV82Debug(state, tracer, s_57_0);
        // S s_57_2: not s_57_1
        let s_57_2: bool = !s_57_1;
        // D s_57_3: write-var gs#15971 <= s_57_2
        fn_state.gs_15971 = s_57_2;
        // N s_57_4: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_58_0: const #() : ()
        let s_58_0: () = ();
        // S s_58_1: call HaveVirtHostExt(s_58_0)
        let s_58_1: bool = HaveVirtHostExt(state, tracer, s_58_0);
        // S s_58_2: not s_58_1
        let s_58_2: bool = !s_58_1;
        // D s_58_3: write-var gs#15970 <= s_58_2
        fn_state.gs_15970 = s_58_2;
        // N s_58_4: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // D s_59_0: read-var bt:u8
        let s_59_0: u8 = fn_state.bt;
        // C s_59_1: const #2s : i
        let s_59_1: i128 = 2;
        // D s_59_2: cast zx s_59_0 -> bv
        let s_59_2: Bits = Bits::new(s_59_0 as u128, 4u16);
        // C s_59_3: const #1s : i64
        let s_59_3: i64 = 1;
        // C s_59_4: cast zx s_59_3 -> i
        let s_59_4: i128 = (i128::try_from(s_59_3).unwrap());
        // C s_59_5: const #1s : i
        let s_59_5: i128 = 1;
        // C s_59_6: add s_59_5 s_59_4
        let s_59_6: i128 = (s_59_5 + s_59_4);
        // D s_59_7: bit-extract s_59_2 s_59_1 s_59_6
        let s_59_7: Bits = (Bits::new(
            ((s_59_2) >> (s_59_1)).value(),
            u16::try_from(s_59_6).unwrap(),
        ));
        // D s_59_8: cast reint s_59_7 -> u8
        let s_59_8: u8 = (s_59_7.value() as u8);
        // D s_59_9: cast zx s_59_8 -> bv
        let s_59_9: Bits = Bits::new(s_59_8 as u128, 2u16);
        // C s_59_10: const #3u : u8
        let s_59_10: u8 = 3;
        // C s_59_11: cast zx s_59_10 -> bv
        let s_59_11: Bits = Bits::new(s_59_10 as u128, 2u16);
        // D s_59_12: cmp-eq s_59_9 s_59_11
        let s_59_12: bool = ((s_59_9) == (s_59_11));
        // D s_59_13: not s_59_12
        let s_59_13: bool = !s_59_12;
        // N s_59_14: branch s_59_13 b61 b60
        if s_59_13 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_60_0: const #1u : u8
        let s_60_0: bool = true;
        // D s_60_1: write-var gs#15962 <= s_60_0
        fn_state.gs_15962 = s_60_0;
        // N s_60_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_61_0: const #0u : u8
        let s_61_0: bool = false;
        // D s_61_1: write-var gs#15962 <= s_61_0
        fn_state.gs_15962 = s_61_0;
        // N s_61_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_62_0: const #1u : u8
        let s_62_0: bool = true;
        // D s_62_1: write-var reserved <= s_62_0
        fn_state.reserved = s_62_0;
        // N s_62_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_63_0: const #432u : u32
        let s_63_0: u32 = 432;
        // D s_63_1: read-reg s_63_0:u8
        let s_63_1: u8 = {
            let value = state.read_register::<u8>(s_63_0 as isize);
            tracer.read_register(s_63_0 as isize, value);
            value
        };
        // C s_63_2: const #2u : u8
        let s_63_2: u8 = 2;
        // D s_63_3: cmp-lt s_63_1 s_63_2
        let s_63_3: bool = ((s_63_1) < (s_63_2));
        // D s_63_4: not s_63_3
        let s_63_4: bool = !s_63_3;
        // D s_63_5: write-var gs#15961 <= s_63_4
        fn_state.gs_15961 = s_63_4;
        // N s_63_6: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_64_0: const #0u : u8
        let s_64_0: bool = false;
        // D s_64_1: write-var gs#15956 <= s_64_0
        fn_state.gs_15956 = s_64_0;
        // N s_64_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_65_0: const #1u : u8
        let s_65_0: bool = true;
        // D s_65_1: write-var reserved <= s_65_0
        fn_state.reserved = s_65_0;
        // N s_65_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // D s_66_0: read-var context_aware:u8
        let s_66_0: bool = fn_state.context_aware;
        // D s_66_1: not s_66_0
        let s_66_1: bool = !s_66_0;
        // D s_66_2: write-var gs#15955 <= s_66_1
        fn_state.gs_15955 = s_66_1;
        // N s_66_3: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_67_0: const #0u : u8
        let s_67_0: bool = false;
        // D s_67_1: write-var gs#15947 <= s_67_0
        fn_state.gs_15947 = s_67_0;
        // N s_67_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeea264718a40d3f4a {
        // C s_68_0: const #1s : i
        let s_68_0: i128 = 1;
        // D s_68_1: read-var b__1:u8
        let s_68_1: u8 = fn_state.b__1;
        // D s_68_2: cast zx s_68_1 -> bv
        let s_68_2: Bits = Bits::new(s_68_1 as u128, 4u16);
        // C s_68_3: const #1s : i64
        let s_68_3: i64 = 1;
        // C s_68_4: cast zx s_68_3 -> i
        let s_68_4: i128 = (i128::try_from(s_68_3).unwrap());
        // C s_68_5: const #0s : i
        let s_68_5: i128 = 0;
        // C s_68_6: add s_68_5 s_68_4
        let s_68_6: i128 = (s_68_5 + s_68_4);
        // D s_68_7: bit-extract s_68_2 s_68_0 s_68_6
        let s_68_7: Bits = (Bits::new(
            ((s_68_2) >> (s_68_0)).value(),
            u16::try_from(s_68_6).unwrap(),
        ));
        // D s_68_8: cast reint s_68_7 -> u8
        let s_68_8: bool = ((s_68_7.value()) != 0);
        // D s_68_9: cast zx s_68_8 -> bv
        let s_68_9: Bits = Bits::new(s_68_8 as u128, 1u16);
        // C s_68_10: const #0u : u8
        let s_68_10: bool = false;
        // C s_68_11: cast zx s_68_10 -> bv
        let s_68_11: Bits = Bits::new(s_68_10 as u128, 1u16);
        // D s_68_12: cmp-eq s_68_9 s_68_11
        let s_68_12: bool = ((s_68_9) == (s_68_11));
        // D s_68_13: write-var gs#15952 <= s_68_12
        fn_state.gs_15952 = s_68_12;
        // N s_68_14: jump b23
        return block_23(state, tracer, fn_state);
    }
}
