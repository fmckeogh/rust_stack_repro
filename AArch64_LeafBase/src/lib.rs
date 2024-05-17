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
use Have52BitPAExt::*;
use u__id::*;
use Zeros::*;
use TGxGranuleBits::*;
use place_subrange::*;
use Have56BitPAExt::*;
use common::*;
pub fn AArch64_LeafBase<T: Tracer>(
    state: &mut State,
    tracer: &T,
    descriptor: Bits,
    d128: bool,
    ds: bool,
    tgx: u32,
    level: i128,
) -> u64 {
    #[derive(Default)]
    struct FunctionState {
        granulebits: i128,
        return_value: u64,
        ga_13320: i64,
        leafsize: i128,
        gs_17820: bool,
        gs_17821: bool,
        leafbase: u64,
        gs_17826: bool,
        gs_17827: bool,
        descriptor: Bits,
        d128: bool,
        ds: bool,
        tgx: u32,
        level: i128,
    }
    let fn_state = FunctionState {
        descriptor,
        d128,
        ds,
        tgx,
        level,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_0_0: const #56s : i
        let s_0_0: i128 = 56;
        // S s_0_1: call Zeros(s_0_0)
        let s_0_1: Bits = Zeros(state, tracer, s_0_0);
        // S s_0_2: cast reint s_0_1 -> u56
        let s_0_2: u64 = (s_0_1.value() as u64);
        // D s_0_3: write-var leafbase <= s_0_2
        fn_state.leafbase = s_0_2;
        // D s_0_4: read-var tgx:u32
        let s_0_4: u32 = fn_state.tgx;
        // D s_0_5: call TGxGranuleBits(s_0_4)
        let s_0_5: i128 = TGxGranuleBits(state, tracer, s_0_4);
        // D s_0_6: write-var granulebits <= s_0_5
        fn_state.granulebits = s_0_5;
        // D s_0_7: read-var d128:u8
        let s_0_7: bool = fn_state.d128;
        // D s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 1u16);
        // C s_0_9: const #1u : u8
        let s_0_9: bool = true;
        // C s_0_10: cast zx s_0_9 -> bv
        let s_0_10: Bits = Bits::new(s_0_9 as u128, 1u16);
        // D s_0_11: cmp-eq s_0_8 s_0_10
        let s_0_11: bool = ((s_0_8) == (s_0_10));
        // N s_0_12: branch s_0_11 b23 b1
        if s_0_11 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_1_0: const #3s : i64
        let s_1_0: i64 = 3;
        // D s_1_1: write-var ga#13320 <= s_1_0
        fn_state.ga_13320 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_2_0: read-var ga#13320:i64
        let s_2_0: i64 = fn_state.ga_13320;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: read-var granulebits:i
        let s_2_2: i128 = fn_state.granulebits;
        // D s_2_3: sub s_2_2 s_2_1
        let s_2_3: i128 = ((s_2_2) - (s_2_1));
        // C s_2_4: const #800u : u32
        let s_2_4: u32 = 800;
        // D s_2_5: read-reg s_2_4:i64
        let s_2_5: i64 = {
            let value = state.read_register::<i64>(s_2_4 as isize);
            tracer.read_register(s_2_4 as isize, value);
            value
        };
        // D s_2_6: cast zx s_2_5 -> i
        let s_2_6: i128 = (i128::try_from(s_2_5).unwrap());
        // D s_2_7: read-var level:i
        let s_2_7: i128 = fn_state.level;
        // D s_2_8: sub s_2_6 s_2_7
        let s_2_8: i128 = ((s_2_6) - (s_2_7));
        // D s_2_9: mul s_2_3 s_2_8
        let s_2_9: i128 = ((s_2_3) * (s_2_8));
        // D s_2_10: read-var granulebits:i
        let s_2_10: i128 = fn_state.granulebits;
        // D s_2_11: add s_2_10 s_2_9
        let s_2_11: i128 = (s_2_10 + s_2_9);
        // D s_2_12: write-var leafsize <= s_2_11
        fn_state.leafsize = s_2_11;
        // D s_2_13: read-var leafsize:i
        let s_2_13: i128 = fn_state.leafsize;
        // D s_2_14: call __id(s_2_13)
        let s_2_14: i128 = u__id(state, tracer, s_2_13);
        // C s_2_15: const #0s : i
        let s_2_15: i128 = 0;
        // D s_2_16: cmp-le s_2_15 s_2_14
        let s_2_16: bool = ((s_2_15) <= (s_2_14));
        // N s_2_17: branch s_2_16 b19 b3
        if s_2_16 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#17821 <= s_3_0
        fn_state.gs_17821 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_4_0: read-var gs#17821:u8
        let s_4_0: bool = fn_state.gs_17821;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // C s_4_2: const #48s : i
        let s_4_2: i128 = 48;
        // C s_4_3: const #47s : i
        let s_4_3: i128 = 47;
        // D s_4_4: read-var descriptor:bv
        let s_4_4: Bits = fn_state.descriptor;
        // D s_4_5: read-var leafsize:i
        let s_4_5: i128 = fn_state.leafsize;
        // D s_4_6: read-var leafsize:i
        let s_4_6: i128 = fn_state.leafsize;
        // D s_4_7: call place_subrange(s_4_2, s_4_4, s_4_3, s_4_5, s_4_6)
        let s_4_7: Bits = place_subrange(
            state,
            tracer,
            s_4_2,
            s_4_4,
            s_4_3,
            s_4_5,
            s_4_6,
        );
        // D s_4_8: cast reint s_4_7 -> u48
        let s_4_8: u64 = (s_4_7.value() as u64);
        // C s_4_9: const #0s : i
        let s_4_9: i128 = 0;
        // D s_4_10: read-var leafbase:u56
        let s_4_10: u64 = fn_state.leafbase;
        // D s_4_11: cast zx s_4_10 -> bv
        let s_4_11: Bits = Bits::new(s_4_10 as u128, 56u16);
        // D s_4_12: cast zx s_4_8 -> bv
        let s_4_12: Bits = Bits::new(s_4_8 as u128, 48u16);
        // C s_4_13: const #47s : i
        let s_4_13: i128 = 47;
        // C s_4_14: const #1u : u64
        let s_4_14: u64 = 1;
        // C s_4_15: cast zx s_4_14 -> bv
        let s_4_15: Bits = Bits::new(s_4_14 as u128, 64u16);
        // C s_4_16: lsl s_4_15 s_4_13
        let s_4_16: Bits = s_4_15 << s_4_13;
        // C s_4_17: sub s_4_16 s_4_15
        let s_4_17: Bits = ((s_4_16) - (s_4_15));
        // D s_4_18: and s_4_12 s_4_17
        let s_4_18: Bits = ((s_4_12) & (s_4_17));
        // D s_4_19: lsl s_4_18 s_4_9
        let s_4_19: Bits = s_4_18 << s_4_9;
        // C s_4_20: lsl s_4_17 s_4_9
        let s_4_20: Bits = s_4_17 << s_4_9;
        // C s_4_21: cmpl s_4_20
        let s_4_21: Bits = !s_4_20;
        // D s_4_22: and s_4_11 s_4_21
        let s_4_22: Bits = ((s_4_11) & (s_4_21));
        // D s_4_23: or s_4_22 s_4_19
        let s_4_23: Bits = ((s_4_22) | (s_4_19));
        // D s_4_24: cast reint s_4_23 -> u56
        let s_4_24: u64 = (s_4_23.value() as u64);
        // D s_4_25: write-var leafbase <= s_4_24
        fn_state.leafbase = s_4_24;
        // C s_4_26: const #() : ()
        let s_4_26: () = ();
        // S s_4_27: call Have56BitPAExt(s_4_26)
        let s_4_27: bool = Have56BitPAExt(state, tracer, s_4_26);
        // N s_4_28: branch s_4_27 b18 b5
        if s_4_27 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#17826 <= s_5_0
        fn_state.gs_17826 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_6_0: read-var gs#17826:u8
        let s_6_0: bool = fn_state.gs_17826;
        // N s_6_1: branch s_6_0 b17 b7
        if s_6_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call Have52BitPAExt(s_7_0)
        let s_7_1: bool = Have52BitPAExt(state, tracer, s_7_0);
        // N s_7_2: branch s_7_1 b16 b8
        if s_7_1 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#17827 <= s_8_0
        fn_state.gs_17827 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_9_0: read-var gs#17827:u8
        let s_9_0: bool = fn_state.gs_17827;
        // N s_9_1: branch s_9_0 b15 b10
        if s_9_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_10_0: read-var ds:u8
        let s_10_0: bool = fn_state.ds;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // C s_10_2: const #1u : u8
        let s_10_2: bool = true;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // N s_10_5: branch s_10_4 b14 b11
        if s_10_4 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_12_0: read-var leafbase:u56
        let s_12_0: u64 = fn_state.leafbase;
        // D s_12_1: write-var return_value <= s_12_0
        fn_state.return_value = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_13_0: read-var return_value:u56
        let s_13_0: u64 = fn_state.return_value;
        // N s_13_1: return s_13_0
        return s_13_0;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_14_0: read-var descriptor:bv
        let s_14_0: Bits = fn_state.descriptor;
        // D s_14_1: size-of s_14_0
        let s_14_1: u16 = s_14_0.length();
        // D s_14_2: cast zx s_14_1 -> i
        let s_14_2: i128 = (i128::try_from(s_14_1).unwrap());
        // C s_14_3: const #49s : i
        let s_14_3: i128 = 49;
        // D s_14_4: cmp-lt s_14_3 s_14_2
        let s_14_4: bool = ((s_14_3) < (s_14_2));
        // N s_14_5: assert s_14_4
        let s_14_5: () = assert!(s_14_4);
        // C s_14_6: const #8s : i
        let s_14_6: i128 = 8;
        // D s_14_7: read-var descriptor:bv
        let s_14_7: Bits = fn_state.descriptor;
        // C s_14_8: const #1s : i64
        let s_14_8: i64 = 1;
        // C s_14_9: cast zx s_14_8 -> i
        let s_14_9: i128 = (i128::try_from(s_14_8).unwrap());
        // C s_14_10: const #1s : i
        let s_14_10: i128 = 1;
        // C s_14_11: add s_14_10 s_14_9
        let s_14_11: i128 = (s_14_10 + s_14_9);
        // D s_14_12: bit-extract s_14_7 s_14_6 s_14_11
        let s_14_12: Bits = (Bits::new(
            ((s_14_7) >> (s_14_6)).value(),
            u16::try_from(s_14_11).unwrap(),
        ));
        // D s_14_13: cast reint s_14_12 -> u8
        let s_14_13: u8 = (s_14_12.value() as u8);
        // C s_14_14: const #48s : i
        let s_14_14: i128 = 48;
        // D s_14_15: read-var descriptor:bv
        let s_14_15: Bits = fn_state.descriptor;
        // C s_14_16: const #1s : i64
        let s_14_16: i64 = 1;
        // C s_14_17: cast zx s_14_16 -> i
        let s_14_17: i128 = (i128::try_from(s_14_16).unwrap());
        // C s_14_18: const #1s : i
        let s_14_18: i128 = 1;
        // C s_14_19: add s_14_18 s_14_17
        let s_14_19: i128 = (s_14_18 + s_14_17);
        // D s_14_20: bit-extract s_14_15 s_14_14 s_14_19
        let s_14_20: Bits = (Bits::new(
            ((s_14_15) >> (s_14_14)).value(),
            u16::try_from(s_14_19).unwrap(),
        ));
        // D s_14_21: cast reint s_14_20 -> u8
        let s_14_21: u8 = (s_14_20.value() as u8);
        // D s_14_22: cast zx s_14_13 -> bv
        let s_14_22: Bits = Bits::new(s_14_13 as u128, 2u16);
        // D s_14_23: cast zx s_14_21 -> bv
        let s_14_23: Bits = Bits::new(s_14_21 as u128, 2u16);
        // D s_14_24: cast reint s_14_22 -> u128
        let s_14_24: u128 = (s_14_22.value() as u128);
        // D s_14_25: size-of s_14_22
        let s_14_25: u16 = s_14_22.length();
        // D s_14_26: cast reint s_14_23 -> u128
        let s_14_26: u128 = (s_14_23.value() as u128);
        // D s_14_27: size-of s_14_23
        let s_14_27: u16 = s_14_23.length();
        // D s_14_28: lsl s_14_24 s_14_27
        let s_14_28: u128 = s_14_24 << s_14_27;
        // D s_14_29: or s_14_28 s_14_26
        let s_14_29: u128 = ((s_14_28) | (s_14_26));
        // D s_14_30: add s_14_25 s_14_27
        let s_14_30: u16 = (s_14_25 + s_14_27);
        // D s_14_31: create-bits s_14_29 s_14_30
        let s_14_31: Bits = Bits::new(s_14_29, s_14_30);
        // D s_14_32: cast reint s_14_31 -> u8
        let s_14_32: u8 = (s_14_31.value() as u8);
        // C s_14_33: const #48s : i
        let s_14_33: i128 = 48;
        // D s_14_34: read-var leafbase:u56
        let s_14_34: u64 = fn_state.leafbase;
        // D s_14_35: cast zx s_14_34 -> bv
        let s_14_35: Bits = Bits::new(s_14_34 as u128, 56u16);
        // D s_14_36: cast zx s_14_32 -> bv
        let s_14_36: Bits = Bits::new(s_14_32 as u128, 4u16);
        // C s_14_37: const #3s : i
        let s_14_37: i128 = 3;
        // C s_14_38: const #1u : u64
        let s_14_38: u64 = 1;
        // C s_14_39: cast zx s_14_38 -> bv
        let s_14_39: Bits = Bits::new(s_14_38 as u128, 64u16);
        // C s_14_40: lsl s_14_39 s_14_37
        let s_14_40: Bits = s_14_39 << s_14_37;
        // C s_14_41: sub s_14_40 s_14_39
        let s_14_41: Bits = ((s_14_40) - (s_14_39));
        // D s_14_42: and s_14_36 s_14_41
        let s_14_42: Bits = ((s_14_36) & (s_14_41));
        // D s_14_43: lsl s_14_42 s_14_33
        let s_14_43: Bits = s_14_42 << s_14_33;
        // C s_14_44: lsl s_14_41 s_14_33
        let s_14_44: Bits = s_14_41 << s_14_33;
        // C s_14_45: cmpl s_14_44
        let s_14_45: Bits = !s_14_44;
        // D s_14_46: and s_14_35 s_14_45
        let s_14_46: Bits = ((s_14_35) & (s_14_45));
        // D s_14_47: or s_14_46 s_14_43
        let s_14_47: Bits = ((s_14_46) | (s_14_43));
        // D s_14_48: cast reint s_14_47 -> u56
        let s_14_48: u64 = (s_14_47.value() as u64);
        // D s_14_49: write-var leafbase <= s_14_48
        fn_state.leafbase = s_14_48;
        // N s_14_50: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_15_0: const #12s : i
        let s_15_0: i128 = 12;
        // D s_15_1: read-var descriptor:bv
        let s_15_1: Bits = fn_state.descriptor;
        // C s_15_2: const #1s : i64
        let s_15_2: i64 = 1;
        // C s_15_3: cast zx s_15_2 -> i
        let s_15_3: i128 = (i128::try_from(s_15_2).unwrap());
        // C s_15_4: const #3s : i
        let s_15_4: i128 = 3;
        // C s_15_5: add s_15_4 s_15_3
        let s_15_5: i128 = (s_15_4 + s_15_3);
        // D s_15_6: bit-extract s_15_1 s_15_0 s_15_5
        let s_15_6: Bits = (Bits::new(
            ((s_15_1) >> (s_15_0)).value(),
            u16::try_from(s_15_5).unwrap(),
        ));
        // D s_15_7: cast reint s_15_6 -> u8
        let s_15_7: u8 = (s_15_6.value() as u8);
        // C s_15_8: const #48s : i
        let s_15_8: i128 = 48;
        // D s_15_9: read-var leafbase:u56
        let s_15_9: u64 = fn_state.leafbase;
        // D s_15_10: cast zx s_15_9 -> bv
        let s_15_10: Bits = Bits::new(s_15_9 as u128, 56u16);
        // D s_15_11: cast zx s_15_7 -> bv
        let s_15_11: Bits = Bits::new(s_15_7 as u128, 4u16);
        // C s_15_12: const #3s : i
        let s_15_12: i128 = 3;
        // C s_15_13: const #1u : u64
        let s_15_13: u64 = 1;
        // C s_15_14: cast zx s_15_13 -> bv
        let s_15_14: Bits = Bits::new(s_15_13 as u128, 64u16);
        // C s_15_15: lsl s_15_14 s_15_12
        let s_15_15: Bits = s_15_14 << s_15_12;
        // C s_15_16: sub s_15_15 s_15_14
        let s_15_16: Bits = ((s_15_15) - (s_15_14));
        // D s_15_17: and s_15_11 s_15_16
        let s_15_17: Bits = ((s_15_11) & (s_15_16));
        // D s_15_18: lsl s_15_17 s_15_8
        let s_15_18: Bits = s_15_17 << s_15_8;
        // C s_15_19: lsl s_15_16 s_15_8
        let s_15_19: Bits = s_15_16 << s_15_8;
        // C s_15_20: cmpl s_15_19
        let s_15_20: Bits = !s_15_19;
        // D s_15_21: and s_15_10 s_15_20
        let s_15_21: Bits = ((s_15_10) & (s_15_20));
        // D s_15_22: or s_15_21 s_15_18
        let s_15_22: Bits = ((s_15_21) | (s_15_18));
        // D s_15_23: cast reint s_15_22 -> u56
        let s_15_23: u64 = (s_15_22.value() as u64);
        // D s_15_24: write-var leafbase <= s_15_23
        fn_state.leafbase = s_15_23;
        // N s_15_25: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_16_0: read-var tgx:u32
        let s_16_0: u32 = fn_state.tgx;
        // C s_16_1: const #2u : u32
        let s_16_1: u32 = 2;
        // D s_16_2: cmp-eq s_16_0 s_16_1
        let s_16_2: bool = ((s_16_0) == (s_16_1));
        // D s_16_3: write-var gs#17827 <= s_16_2
        fn_state.gs_17827 = s_16_2;
        // N s_16_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_17_0: read-var descriptor:bv
        let s_17_0: Bits = fn_state.descriptor;
        // D s_17_1: size-of s_17_0
        let s_17_1: u16 = s_17_0.length();
        // D s_17_2: cast zx s_17_1 -> i
        let s_17_2: i128 = (i128::try_from(s_17_1).unwrap());
        // C s_17_3: const #55s : i
        let s_17_3: i128 = 55;
        // D s_17_4: cmp-lt s_17_3 s_17_2
        let s_17_4: bool = ((s_17_3) < (s_17_2));
        // N s_17_5: assert s_17_4
        let s_17_5: () = assert!(s_17_4);
        // C s_17_6: const #48s : i
        let s_17_6: i128 = 48;
        // D s_17_7: read-var descriptor:bv
        let s_17_7: Bits = fn_state.descriptor;
        // C s_17_8: const #1s : i64
        let s_17_8: i64 = 1;
        // C s_17_9: cast zx s_17_8 -> i
        let s_17_9: i128 = (i128::try_from(s_17_8).unwrap());
        // C s_17_10: const #7s : i
        let s_17_10: i128 = 7;
        // C s_17_11: add s_17_10 s_17_9
        let s_17_11: i128 = (s_17_10 + s_17_9);
        // D s_17_12: bit-extract s_17_7 s_17_6 s_17_11
        let s_17_12: Bits = (Bits::new(
            ((s_17_7) >> (s_17_6)).value(),
            u16::try_from(s_17_11).unwrap(),
        ));
        // D s_17_13: cast reint s_17_12 -> u8
        let s_17_13: u8 = (s_17_12.value() as u8);
        // C s_17_14: const #48s : i
        let s_17_14: i128 = 48;
        // D s_17_15: read-var leafbase:u56
        let s_17_15: u64 = fn_state.leafbase;
        // D s_17_16: cast zx s_17_15 -> bv
        let s_17_16: Bits = Bits::new(s_17_15 as u128, 56u16);
        // D s_17_17: cast zx s_17_13 -> bv
        let s_17_17: Bits = Bits::new(s_17_13 as u128, 8u16);
        // C s_17_18: const #7s : i
        let s_17_18: i128 = 7;
        // C s_17_19: const #1u : u64
        let s_17_19: u64 = 1;
        // C s_17_20: cast zx s_17_19 -> bv
        let s_17_20: Bits = Bits::new(s_17_19 as u128, 64u16);
        // C s_17_21: lsl s_17_20 s_17_18
        let s_17_21: Bits = s_17_20 << s_17_18;
        // C s_17_22: sub s_17_21 s_17_20
        let s_17_22: Bits = ((s_17_21) - (s_17_20));
        // D s_17_23: and s_17_17 s_17_22
        let s_17_23: Bits = ((s_17_17) & (s_17_22));
        // D s_17_24: lsl s_17_23 s_17_14
        let s_17_24: Bits = s_17_23 << s_17_14;
        // C s_17_25: lsl s_17_22 s_17_14
        let s_17_25: Bits = s_17_22 << s_17_14;
        // C s_17_26: cmpl s_17_25
        let s_17_26: Bits = !s_17_25;
        // D s_17_27: and s_17_16 s_17_26
        let s_17_27: Bits = ((s_17_16) & (s_17_26));
        // D s_17_28: or s_17_27 s_17_24
        let s_17_28: Bits = ((s_17_27) | (s_17_24));
        // D s_17_29: cast reint s_17_28 -> u56
        let s_17_29: u64 = (s_17_28.value() as u64);
        // D s_17_30: write-var leafbase <= s_17_29
        fn_state.leafbase = s_17_29;
        // D s_17_31: read-var leafbase:u56
        let s_17_31: u64 = fn_state.leafbase;
        // D s_17_32: write-var return_value <= s_17_31
        fn_state.return_value = s_17_31;
        // N s_17_33: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_18_0: read-var d128:u8
        let s_18_0: bool = fn_state.d128;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 1u16);
        // C s_18_2: const #1u : u8
        let s_18_2: bool = true;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // D s_18_5: write-var gs#17826 <= s_18_4
        fn_state.gs_17826 = s_18_4;
        // N s_18_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_19_0: read-var leafsize:i
        let s_19_0: i128 = fn_state.leafsize;
        // D s_19_1: call __id(s_19_0)
        let s_19_1: i128 = u__id(state, tracer, s_19_0);
        // C s_19_2: const #47s : i
        let s_19_2: i128 = 47;
        // D s_19_3: cmp-le s_19_1 s_19_2
        let s_19_3: bool = ((s_19_1) <= (s_19_2));
        // N s_19_4: branch s_19_3 b22 b20
        if s_19_3 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var gs#17820 <= s_20_0
        fn_state.gs_17820 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_21_0: read-var gs#17820:u8
        let s_21_0: bool = fn_state.gs_17820;
        // D s_21_1: write-var gs#17821 <= s_21_0
        fn_state.gs_17821 = s_21_0;
        // N s_21_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_22_0: read-var descriptor:bv
        let s_22_0: Bits = fn_state.descriptor;
        // D s_22_1: size-of s_22_0
        let s_22_1: u16 = s_22_0.length();
        // D s_22_2: cast zx s_22_1 -> i
        let s_22_2: i128 = (i128::try_from(s_22_1).unwrap());
        // C s_22_3: const #47s : i
        let s_22_3: i128 = 47;
        // D s_22_4: cmp-lt s_22_3 s_22_2
        let s_22_4: bool = ((s_22_3) < (s_22_2));
        // D s_22_5: write-var gs#17820 <= s_22_4
        fn_state.gs_17820 = s_22_4;
        // N s_22_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_23_0: const #4s : i64
        let s_23_0: i64 = 4;
        // D s_23_1: write-var ga#13320 <= s_23_0
        fn_state.ga_13320 = s_23_0;
        // N s_23_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
