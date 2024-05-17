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
use AbovePPS::*;
use IsZero::*;
use u__UNKNOWN_GPTTable::*;
use is_zero_subrange::*;
use GPTL0Size::*;
use Zeros::*;
use Unreachable::*;
use common::*;
pub fn DecodeGPTTable<T: Tracer>(
    state: &mut State,
    tracer: &T,
    pgs: u32,
    entry: u64,
) -> ProductType3121c658f1e84c22 {
    #[derive(Default)]
    struct FunctionState {
        p: i64,
        return_value: ProductType3121c658f1e84c22,
        l0sz: i128,
        result: ProductTypeecb3a6c821d7caab,
        pgs: u32,
        entry: u64,
    }
    let fn_state = FunctionState {
        pgs,
        entry,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3121c658f1e84c22 {
        // C s_0_0: const #0s : i
        let s_0_0: i128 = 0;
        // D s_0_1: read-var entry:u64
        let s_0_1: u64 = fn_state.entry;
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 64u16);
        // C s_0_3: const #1s : i64
        let s_0_3: i64 = 1;
        // C s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (i128::try_from(s_0_3).unwrap());
        // C s_0_5: const #3s : i
        let s_0_5: i128 = 3;
        // C s_0_6: add s_0_5 s_0_4
        let s_0_6: i128 = (s_0_5 + s_0_4);
        // D s_0_7: bit-extract s_0_2 s_0_0 s_0_6
        let s_0_7: Bits = (Bits::new(
            ((s_0_2) >> (s_0_0)).value(),
            u16::try_from(s_0_6).unwrap(),
        ));
        // D s_0_8: cast reint s_0_7 -> u8
        let s_0_8: u8 = (s_0_7.value() as u8);
        // D s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 4u16);
        // C s_0_10: const #848u : u32
        let s_0_10: u32 = 848;
        // D s_0_11: read-reg s_0_10:u8
        let s_0_11: u8 = {
            let value = state.read_register::<u8>(s_0_10 as isize);
            tracer.read_register(s_0_10 as isize, value);
            value
        };
        // D s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 4u16);
        // D s_0_13: cmp-eq s_0_9 s_0_12
        let s_0_13: bool = ((s_0_9) == (s_0_12));
        // N s_0_14: assert s_0_13
        let s_0_14: () = assert!(s_0_13);
        // C s_0_15: const #52s : i
        let s_0_15: i128 = 52;
        // D s_0_16: read-var entry:u64
        let s_0_16: u64 = fn_state.entry;
        // D s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 64u16);
        // C s_0_18: const #1s : i64
        let s_0_18: i64 = 1;
        // C s_0_19: cast zx s_0_18 -> i
        let s_0_19: i128 = (i128::try_from(s_0_18).unwrap());
        // C s_0_20: const #11s : i
        let s_0_20: i128 = 11;
        // C s_0_21: add s_0_20 s_0_19
        let s_0_21: i128 = (s_0_20 + s_0_19);
        // D s_0_22: bit-extract s_0_17 s_0_15 s_0_21
        let s_0_22: Bits = (Bits::new(
            ((s_0_17) >> (s_0_15)).value(),
            u16::try_from(s_0_21).unwrap(),
        ));
        // D s_0_23: cast reint s_0_22 -> u12
        let s_0_23: u16 = (s_0_22.value() as u16);
        // C s_0_24: const #4s : i
        let s_0_24: i128 = 4;
        // D s_0_25: read-var entry:u64
        let s_0_25: u64 = fn_state.entry;
        // D s_0_26: cast zx s_0_25 -> bv
        let s_0_26: Bits = Bits::new(s_0_25 as u128, 64u16);
        // C s_0_27: const #1s : i64
        let s_0_27: i64 = 1;
        // C s_0_28: cast zx s_0_27 -> i
        let s_0_28: i128 = (i128::try_from(s_0_27).unwrap());
        // C s_0_29: const #7s : i
        let s_0_29: i128 = 7;
        // C s_0_30: add s_0_29 s_0_28
        let s_0_30: i128 = (s_0_29 + s_0_28);
        // D s_0_31: bit-extract s_0_26 s_0_24 s_0_30
        let s_0_31: Bits = (Bits::new(
            ((s_0_26) >> (s_0_24)).value(),
            u16::try_from(s_0_30).unwrap(),
        ));
        // D s_0_32: cast reint s_0_31 -> u8
        let s_0_32: u8 = (s_0_31.value() as u8);
        // D s_0_33: cast zx s_0_23 -> bv
        let s_0_33: Bits = Bits::new(s_0_23 as u128, 12u16);
        // D s_0_34: cast zx s_0_32 -> bv
        let s_0_34: Bits = Bits::new(s_0_32 as u128, 8u16);
        // D s_0_35: cast reint s_0_33 -> u128
        let s_0_35: u128 = (s_0_33.value() as u128);
        // D s_0_36: size-of s_0_33
        let s_0_36: u16 = s_0_33.length();
        // D s_0_37: cast reint s_0_34 -> u128
        let s_0_37: u128 = (s_0_34.value() as u128);
        // D s_0_38: size-of s_0_34
        let s_0_38: u16 = s_0_34.length();
        // D s_0_39: lsl s_0_35 s_0_38
        let s_0_39: u128 = s_0_35 << s_0_38;
        // D s_0_40: or s_0_39 s_0_37
        let s_0_40: u128 = ((s_0_39) | (s_0_37));
        // D s_0_41: add s_0_36 s_0_38
        let s_0_41: u16 = (s_0_36 + s_0_38);
        // D s_0_42: create-bits s_0_40 s_0_41
        let s_0_42: Bits = Bits::new(s_0_40, s_0_41);
        // D s_0_43: cast reint s_0_42 -> u20
        let s_0_43: u32 = (s_0_42.value() as u32);
        // D s_0_44: cast zx s_0_43 -> bv
        let s_0_44: Bits = Bits::new(s_0_43 as u128, 20u16);
        // D s_0_45: call IsZero(s_0_44)
        let s_0_45: bool = IsZero(state, tracer, s_0_44);
        // D s_0_46: not s_0_45
        let s_0_46: bool = !s_0_45;
        // N s_0_47: branch s_0_46 b21 b1
        if s_0_46 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3121c658f1e84c22 {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call GPTL0Size(s_1_0)
        let s_1_1: i128 = GPTL0Size(state, tracer, s_1_0);
        // D s_1_2: write-var l0sz <= s_1_1
        fn_state.l0sz = s_1_1;
        // C s_1_3: const #12s : i64
        let s_1_3: i64 = 12;
        // D s_1_4: write-var p <= s_1_3
        fn_state.p = s_1_3;
        // C s_1_5: const #0u : u32
        let s_1_5: u32 = 0;
        // D s_1_6: read-var pgs:u32
        let s_1_6: u32 = fn_state.pgs;
        // D s_1_7: cmp-eq s_1_5 s_1_6
        let s_1_7: bool = ((s_1_5) == (s_1_6));
        // D s_1_8: not s_1_7
        let s_1_8: bool = !s_1_7;
        // N s_1_9: branch s_1_8 b16 b2
        if s_1_8 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3121c658f1e84c22 {
        // C s_2_0: const #12s : i64
        let s_2_0: i64 = 12;
        // D s_2_1: write-var p <= s_2_0
        fn_state.p = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3121c658f1e84c22 {
        // D s_3_0: read-var p:i64
        let s_3_0: i64 = fn_state.p;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var l0sz:i
        let s_3_2: i128 = fn_state.l0sz;
        // D s_3_3: sub s_3_2 s_3_1
        let s_3_3: i128 = ((s_3_2) - (s_3_1));
        // C s_3_4: const #2s : i
        let s_3_4: i128 = 2;
        // D s_3_5: sub s_3_3 s_3_4
        let s_3_5: i128 = ((s_3_3) - (s_3_4));
        // C s_3_6: const #12s : i
        let s_3_6: i128 = 12;
        // D s_3_7: read-var entry:u64
        let s_3_7: u64 = fn_state.entry;
        // D s_3_8: cast zx s_3_7 -> bv
        let s_3_8: Bits = Bits::new(s_3_7 as u128, 64u16);
        // D s_3_9: call is_zero_subrange(s_3_8, s_3_5, s_3_6)
        let s_3_9: bool = is_zero_subrange(state, tracer, s_3_8, s_3_5, s_3_6);
        // D s_3_10: not s_3_9
        let s_3_10: bool = !s_3_9;
        // N s_3_11: branch s_3_10 b15 b4
        if s_3_10 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3121c658f1e84c22 {
        // C s_4_0: const #0u : u32
        let s_4_0: u32 = 0;
        // D s_4_1: read-var pgs:u32
        let s_4_1: u32 = fn_state.pgs;
        // D s_4_2: cmp-eq s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) == (s_4_1));
        // D s_4_3: not s_4_2
        let s_4_3: bool = !s_4_2;
        // N s_4_4: branch s_4_3 b10 b5
        if s_4_3 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3121c658f1e84c22 {
        // C s_5_0: const #17s : i
        let s_5_0: i128 = 17;
        // D s_5_1: read-var entry:u64
        let s_5_1: u64 = fn_state.entry;
        // D s_5_2: cast zx s_5_1 -> bv
        let s_5_2: Bits = Bits::new(s_5_1 as u128, 64u16);
        // C s_5_3: const #1s : i64
        let s_5_3: i64 = 1;
        // C s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // C s_5_5: const #38s : i
        let s_5_5: i128 = 38;
        // C s_5_6: add s_5_5 s_5_4
        let s_5_6: i128 = (s_5_5 + s_5_4);
        // D s_5_7: bit-extract s_5_2 s_5_0 s_5_6
        let s_5_7: Bits = (Bits::new(
            ((s_5_2) >> (s_5_0)).value(),
            u16::try_from(s_5_6).unwrap(),
        ));
        // D s_5_8: cast reint s_5_7 -> u39
        let s_5_8: u64 = (s_5_7.value() as u64);
        // C s_5_9: const #17s : i
        let s_5_9: i128 = 17;
        // S s_5_10: call Zeros(s_5_9)
        let s_5_10: Bits = Zeros(state, tracer, s_5_9);
        // S s_5_11: cast reint s_5_10 -> u17
        let s_5_11: u32 = (s_5_10.value() as u32);
        // D s_5_12: cast zx s_5_8 -> bv
        let s_5_12: Bits = Bits::new(s_5_8 as u128, 39u16);
        // S s_5_13: cast zx s_5_11 -> bv
        let s_5_13: Bits = Bits::new(s_5_11 as u128, 17u16);
        // D s_5_14: cast reint s_5_12 -> u128
        let s_5_14: u128 = (s_5_12.value() as u128);
        // D s_5_15: size-of s_5_12
        let s_5_15: u16 = s_5_12.length();
        // S s_5_16: cast reint s_5_13 -> u128
        let s_5_16: u128 = (s_5_13.value() as u128);
        // D s_5_17: size-of s_5_13
        let s_5_17: u16 = s_5_13.length();
        // D s_5_18: lsl s_5_14 s_5_17
        let s_5_18: u128 = s_5_14 << s_5_17;
        // D s_5_19: or s_5_18 s_5_16
        let s_5_19: u128 = ((s_5_18) | (s_5_16));
        // D s_5_20: add s_5_15 s_5_17
        let s_5_20: u16 = (s_5_15 + s_5_17);
        // D s_5_21: create-bits s_5_19 s_5_20
        let s_5_21: Bits = Bits::new(s_5_19, s_5_20);
        // D s_5_22: cast reint s_5_21 -> u56
        let s_5_22: u64 = (s_5_21.value() as u64);
        // D s_5_23: write-var result.0 <= s_5_22
        fn_state.result._0 = s_5_22;
        // N s_5_24: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3121c658f1e84c22 {
        // D s_6_0: read-var result.0:struct
        let s_6_0: u64 = fn_state.result._0;
        // D s_6_1: call AbovePPS(s_6_0)
        let s_6_1: bool = AbovePPS(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b9 b7
        if s_6_1 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3121c658f1e84c22 {
        // C s_7_0: const #0u : u32
        let s_7_0: u32 = 0;
        // D s_7_1: read-var result:struct
        let s_7_1: ProductTypeecb3a6c821d7caab = fn_state.result;
        // D s_7_2: create-product struct = ["s_7_0", "s_7_1"]
        let s_7_2: ProductType3121c658f1e84c22 = ProductType3121c658f1e84c22 {
            _0: s_7_0,
            _1: s_7_1,
        };
        // D s_7_3: write-var return_value <= s_7_2
        fn_state.return_value = s_7_2;
        // N s_7_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3121c658f1e84c22 {
        // D s_8_0: read-var return_value:struct
        let s_8_0: ProductType3121c658f1e84c22 = fn_state.return_value;
        // N s_8_1: return s_8_0
        return s_8_0;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3121c658f1e84c22 {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call __UNKNOWN_GPTTable(s_9_0)
        let s_9_1: ProductTypeecb3a6c821d7caab = u__UNKNOWN_GPTTable(
            state,
            tracer,
            s_9_0,
        );
        // C s_9_2: const #1u : u32
        let s_9_2: u32 = 1;
        // D s_9_3: create-product struct = ["s_9_2", "s_9_1"]
        let s_9_3: ProductType3121c658f1e84c22 = ProductType3121c658f1e84c22 {
            _0: s_9_2,
            _1: s_9_1,
        };
        // D s_9_4: write-var return_value <= s_9_3
        fn_state.return_value = s_9_3;
        // N s_9_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3121c658f1e84c22 {
        // C s_10_0: const #1u : u32
        let s_10_0: u32 = 1;
        // D s_10_1: read-var pgs:u32
        let s_10_1: u32 = fn_state.pgs;
        // D s_10_2: cmp-eq s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) == (s_10_1));
        // D s_10_3: not s_10_2
        let s_10_3: bool = !s_10_2;
        // N s_10_4: branch s_10_3 b12 b11
        if s_10_3 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3121c658f1e84c22 {
        // C s_11_0: const #15s : i
        let s_11_0: i128 = 15;
        // D s_11_1: read-var entry:u64
        let s_11_1: u64 = fn_state.entry;
        // D s_11_2: cast zx s_11_1 -> bv
        let s_11_2: Bits = Bits::new(s_11_1 as u128, 64u16);
        // C s_11_3: const #1s : i64
        let s_11_3: i64 = 1;
        // C s_11_4: cast zx s_11_3 -> i
        let s_11_4: i128 = (i128::try_from(s_11_3).unwrap());
        // C s_11_5: const #40s : i
        let s_11_5: i128 = 40;
        // C s_11_6: add s_11_5 s_11_4
        let s_11_6: i128 = (s_11_5 + s_11_4);
        // D s_11_7: bit-extract s_11_2 s_11_0 s_11_6
        let s_11_7: Bits = (Bits::new(
            ((s_11_2) >> (s_11_0)).value(),
            u16::try_from(s_11_6).unwrap(),
        ));
        // D s_11_8: cast reint s_11_7 -> u41
        let s_11_8: u64 = (s_11_7.value() as u64);
        // C s_11_9: const #15s : i
        let s_11_9: i128 = 15;
        // S s_11_10: call Zeros(s_11_9)
        let s_11_10: Bits = Zeros(state, tracer, s_11_9);
        // S s_11_11: cast reint s_11_10 -> u15
        let s_11_11: u16 = (s_11_10.value() as u16);
        // D s_11_12: cast zx s_11_8 -> bv
        let s_11_12: Bits = Bits::new(s_11_8 as u128, 41u16);
        // S s_11_13: cast zx s_11_11 -> bv
        let s_11_13: Bits = Bits::new(s_11_11 as u128, 15u16);
        // D s_11_14: cast reint s_11_12 -> u128
        let s_11_14: u128 = (s_11_12.value() as u128);
        // D s_11_15: size-of s_11_12
        let s_11_15: u16 = s_11_12.length();
        // S s_11_16: cast reint s_11_13 -> u128
        let s_11_16: u128 = (s_11_13.value() as u128);
        // D s_11_17: size-of s_11_13
        let s_11_17: u16 = s_11_13.length();
        // D s_11_18: lsl s_11_14 s_11_17
        let s_11_18: u128 = s_11_14 << s_11_17;
        // D s_11_19: or s_11_18 s_11_16
        let s_11_19: u128 = ((s_11_18) | (s_11_16));
        // D s_11_20: add s_11_15 s_11_17
        let s_11_20: u16 = (s_11_15 + s_11_17);
        // D s_11_21: create-bits s_11_19 s_11_20
        let s_11_21: Bits = Bits::new(s_11_19, s_11_20);
        // D s_11_22: cast reint s_11_21 -> u56
        let s_11_22: u64 = (s_11_21.value() as u64);
        // D s_11_23: write-var result.0 <= s_11_22
        fn_state.result._0 = s_11_22;
        // N s_11_24: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3121c658f1e84c22 {
        // C s_12_0: const #2u : u32
        let s_12_0: u32 = 2;
        // D s_12_1: read-var pgs:u32
        let s_12_1: u32 = fn_state.pgs;
        // D s_12_2: cmp-eq s_12_0 s_12_1
        let s_12_2: bool = ((s_12_0) == (s_12_1));
        // D s_12_3: not s_12_2
        let s_12_3: bool = !s_12_2;
        // N s_12_4: branch s_12_3 b14 b13
        if s_12_3 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3121c658f1e84c22 {
        // C s_13_0: const #13s : i
        let s_13_0: i128 = 13;
        // D s_13_1: read-var entry:u64
        let s_13_1: u64 = fn_state.entry;
        // D s_13_2: cast zx s_13_1 -> bv
        let s_13_2: Bits = Bits::new(s_13_1 as u128, 64u16);
        // C s_13_3: const #1s : i64
        let s_13_3: i64 = 1;
        // C s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // C s_13_5: const #42s : i
        let s_13_5: i128 = 42;
        // C s_13_6: add s_13_5 s_13_4
        let s_13_6: i128 = (s_13_5 + s_13_4);
        // D s_13_7: bit-extract s_13_2 s_13_0 s_13_6
        let s_13_7: Bits = (Bits::new(
            ((s_13_2) >> (s_13_0)).value(),
            u16::try_from(s_13_6).unwrap(),
        ));
        // D s_13_8: cast reint s_13_7 -> u43
        let s_13_8: u64 = (s_13_7.value() as u64);
        // C s_13_9: const #13s : i
        let s_13_9: i128 = 13;
        // S s_13_10: call Zeros(s_13_9)
        let s_13_10: Bits = Zeros(state, tracer, s_13_9);
        // S s_13_11: cast reint s_13_10 -> u13
        let s_13_11: u16 = (s_13_10.value() as u16);
        // D s_13_12: cast zx s_13_8 -> bv
        let s_13_12: Bits = Bits::new(s_13_8 as u128, 43u16);
        // S s_13_13: cast zx s_13_11 -> bv
        let s_13_13: Bits = Bits::new(s_13_11 as u128, 13u16);
        // D s_13_14: cast reint s_13_12 -> u128
        let s_13_14: u128 = (s_13_12.value() as u128);
        // D s_13_15: size-of s_13_12
        let s_13_15: u16 = s_13_12.length();
        // S s_13_16: cast reint s_13_13 -> u128
        let s_13_16: u128 = (s_13_13.value() as u128);
        // D s_13_17: size-of s_13_13
        let s_13_17: u16 = s_13_13.length();
        // D s_13_18: lsl s_13_14 s_13_17
        let s_13_18: u128 = s_13_14 << s_13_17;
        // D s_13_19: or s_13_18 s_13_16
        let s_13_19: u128 = ((s_13_18) | (s_13_16));
        // D s_13_20: add s_13_15 s_13_17
        let s_13_20: u16 = (s_13_15 + s_13_17);
        // D s_13_21: create-bits s_13_19 s_13_20
        let s_13_21: Bits = Bits::new(s_13_19, s_13_20);
        // D s_13_22: cast reint s_13_21 -> u56
        let s_13_22: u64 = (s_13_21.value() as u64);
        // D s_13_23: write-var result.0 <= s_13_22
        fn_state.result._0 = s_13_22;
        // N s_13_24: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3121c658f1e84c22 {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call Unreachable(s_14_0)
        let s_14_1: () = Unreachable(state, tracer, s_14_0);
        // N s_14_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3121c658f1e84c22 {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call __UNKNOWN_GPTTable(s_15_0)
        let s_15_1: ProductTypeecb3a6c821d7caab = u__UNKNOWN_GPTTable(
            state,
            tracer,
            s_15_0,
        );
        // C s_15_2: const #2u : u32
        let s_15_2: u32 = 2;
        // D s_15_3: create-product struct = ["s_15_2", "s_15_1"]
        let s_15_3: ProductType3121c658f1e84c22 = ProductType3121c658f1e84c22 {
            _0: s_15_2,
            _1: s_15_1,
        };
        // D s_15_4: write-var return_value <= s_15_3
        fn_state.return_value = s_15_3;
        // N s_15_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3121c658f1e84c22 {
        // C s_16_0: const #1u : u32
        let s_16_0: u32 = 1;
        // D s_16_1: read-var pgs:u32
        let s_16_1: u32 = fn_state.pgs;
        // D s_16_2: cmp-eq s_16_0 s_16_1
        let s_16_2: bool = ((s_16_0) == (s_16_1));
        // D s_16_3: not s_16_2
        let s_16_3: bool = !s_16_2;
        // N s_16_4: branch s_16_3 b18 b17
        if s_16_3 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3121c658f1e84c22 {
        // C s_17_0: const #14s : i64
        let s_17_0: i64 = 14;
        // D s_17_1: write-var p <= s_17_0
        fn_state.p = s_17_0;
        // N s_17_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3121c658f1e84c22 {
        // C s_18_0: const #2u : u32
        let s_18_0: u32 = 2;
        // D s_18_1: read-var pgs:u32
        let s_18_1: u32 = fn_state.pgs;
        // D s_18_2: cmp-eq s_18_0 s_18_1
        let s_18_2: bool = ((s_18_0) == (s_18_1));
        // D s_18_3: not s_18_2
        let s_18_3: bool = !s_18_2;
        // N s_18_4: branch s_18_3 b20 b19
        if s_18_3 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3121c658f1e84c22 {
        // C s_19_0: const #16s : i64
        let s_19_0: i64 = 16;
        // D s_19_1: write-var p <= s_19_0
        fn_state.p = s_19_0;
        // N s_19_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3121c658f1e84c22 {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call Unreachable(s_20_0)
        let s_20_1: () = Unreachable(state, tracer, s_20_0);
        // N s_20_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3121c658f1e84c22 {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call __UNKNOWN_GPTTable(s_21_0)
        let s_21_1: ProductTypeecb3a6c821d7caab = u__UNKNOWN_GPTTable(
            state,
            tracer,
            s_21_0,
        );
        // C s_21_2: const #2u : u32
        let s_21_2: u32 = 2;
        // D s_21_3: create-product struct = ["s_21_2", "s_21_1"]
        let s_21_3: ProductType3121c658f1e84c22 = ProductType3121c658f1e84c22 {
            _0: s_21_2,
            _1: s_21_1,
        };
        // D s_21_4: write-var return_value <= s_21_3
        fn_state.return_value = s_21_3;
        // N s_21_5: jump b8
        return block_8(state, tracer, fn_state);
    }
}
