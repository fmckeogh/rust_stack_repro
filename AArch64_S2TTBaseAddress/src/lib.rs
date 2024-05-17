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
use AArch64_S2StartLevel::*;
use Have56BitPAExt::*;
use u_shl_int_general::*;
use Align_bits::*;
use Zeros::*;
use Have52BitPAExt::*;
use TGxGranuleBits::*;
use AArch64_IASize::*;
use common::*;
pub fn AArch64_S2TTBaseAddress<T: Tracer>(
    state: &mut State,
    tracer: &T,
    walkparams: ProductTypeb05ce25a107f0c5e,
    paspace: u32,
    ttbr: Bits,
) -> u64 {
    #[derive(Default)]
    struct FunctionState {
        tablebase: u64,
        gs_19087: bool,
        gs_19085: bool,
        ga_14277: i64,
        granulebits: i128,
        tsize: i128,
        gs_19089: bool,
        gs_19088: bool,
        iasize: i128,
        walkparams: ProductTypeb05ce25a107f0c5e,
        paspace: u32,
        ttbr: Bits,
    }
    let fn_state = FunctionState {
        walkparams,
        paspace,
        ttbr,
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
        // D s_0_3: write-var tablebase <= s_0_2
        fn_state.tablebase = s_0_2;
        // D s_0_4: read-var walkparams.29:struct
        let s_0_4: u8 = fn_state.walkparams._29;
        // D s_0_5: call AArch64_IASize(s_0_4)
        let s_0_5: i128 = AArch64_IASize(state, tracer, s_0_4);
        // D s_0_6: write-var iasize <= s_0_5
        fn_state.iasize = s_0_5;
        // D s_0_7: read-var walkparams.26:struct
        let s_0_7: u32 = fn_state.walkparams._26;
        // D s_0_8: call TGxGranuleBits(s_0_7)
        let s_0_8: i128 = TGxGranuleBits(state, tracer, s_0_7);
        // D s_0_9: write-var granulebits <= s_0_8
        fn_state.granulebits = s_0_8;
        // D s_0_10: read-var walkparams.2:struct
        let s_0_10: bool = fn_state.walkparams._2;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 1u16);
        // C s_0_12: const #1u : u8
        let s_0_12: bool = true;
        // C s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 1u16);
        // D s_0_14: cmp-eq s_0_11 s_0_13
        let s_0_14: bool = ((s_0_11) == (s_0_13));
        // N s_0_15: branch s_0_14 b22 b1
        if s_0_14 {
            return block_22(state, tracer, fn_state);
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
        // D s_1_1: write-var ga#14277 <= s_1_0
        fn_state.ga_14277 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_2_0: read-var ga#14277:i64
        let s_2_0: i64 = fn_state.ga_14277;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: read-var granulebits:i
        let s_2_2: i128 = fn_state.granulebits;
        // D s_2_3: sub s_2_2 s_2_1
        let s_2_3: i128 = ((s_2_2) - (s_2_1));
        // D s_2_4: read-var walkparams:struct
        let s_2_4: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_2_5: call AArch64_S2StartLevel(s_2_4)
        let s_2_5: i128 = AArch64_S2StartLevel(state, tracer, s_2_4);
        // C s_2_6: const #800u : u32
        let s_2_6: u32 = 800;
        // D s_2_7: read-reg s_2_6:i64
        let s_2_7: i64 = {
            let value = state.read_register::<i64>(s_2_6 as isize);
            tracer.read_register(s_2_6 as isize, value);
            value
        };
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (i128::try_from(s_2_7).unwrap());
        // D s_2_9: sub s_2_8 s_2_5
        let s_2_9: i128 = ((s_2_8) - (s_2_5));
        // D s_2_10: mul s_2_9 s_2_3
        let s_2_10: i128 = ((s_2_9) * (s_2_3));
        // D s_2_11: read-var granulebits:i
        let s_2_11: i128 = fn_state.granulebits;
        // D s_2_12: add s_2_10 s_2_11
        let s_2_12: i128 = (s_2_10 + s_2_11);
        // D s_2_13: read-var iasize:i
        let s_2_13: i128 = fn_state.iasize;
        // D s_2_14: sub s_2_13 s_2_12
        let s_2_14: i128 = ((s_2_13) - (s_2_12));
        // D s_2_15: cast zx s_2_0 -> i
        let s_2_15: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_16: add s_2_14 s_2_15
        let s_2_16: i128 = (s_2_14 + s_2_15);
        // D s_2_17: write-var tsize <= s_2_16
        fn_state.tsize = s_2_16;
        // C s_2_18: const #() : ()
        let s_2_18: () = ();
        // S s_2_19: call Have56BitPAExt(s_2_18)
        let s_2_19: bool = Have56BitPAExt(state, tracer, s_2_18);
        // N s_2_20: branch s_2_19 b21 b3
        if s_2_19 {
            return block_21(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#19085 <= s_3_0
        fn_state.gs_19085 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_4_0: read-var gs#19085:u8
        let s_4_0: bool = fn_state.gs_19085;
        // N s_4_1: branch s_4_0 b18 b5
        if s_4_0 {
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
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call Have52BitPAExt(s_5_0)
        let s_5_1: bool = Have52BitPAExt(state, tracer, s_5_0);
        // N s_5_2: branch s_5_1 b17 b6
        if s_5_1 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#19087 <= s_6_0
        fn_state.gs_19087 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_7_0: read-var gs#19087:u8
        let s_7_0: bool = fn_state.gs_19087;
        // N s_7_1: branch s_7_0 b16 b8
        if s_7_0 {
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
        // D s_8_1: write-var gs#19088 <= s_8_0
        fn_state.gs_19088 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_9_0: read-var gs#19088:u8
        let s_9_0: bool = fn_state.gs_19088;
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
        // D s_10_0: read-var walkparams.3:struct
        let s_10_0: bool = fn_state.walkparams._3;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // C s_10_2: const #1u : u8
        let s_10_2: bool = true;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // D s_10_5: write-var gs#19089 <= s_10_4
        fn_state.gs_19089 = s_10_4;
        // N s_10_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_11_0: read-var gs#19089:u8
        let s_11_0: bool = fn_state.gs_19089;
        // N s_11_1: branch s_11_0 b14 b12
        if s_11_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_12_0: read-var ttbr:bv
        let s_12_0: Bits = fn_state.ttbr;
        // D s_12_1: size-of s_12_0
        let s_12_1: u16 = s_12_0.length();
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // C s_12_3: const #47s : i
        let s_12_3: i128 = 47;
        // D s_12_4: cmp-lt s_12_3 s_12_2
        let s_12_4: bool = ((s_12_3) < (s_12_2));
        // N s_12_5: assert s_12_4
        let s_12_5: () = assert!(s_12_4);
        // C s_12_6: const #1s : i
        let s_12_6: i128 = 1;
        // D s_12_7: read-var ttbr:bv
        let s_12_7: Bits = fn_state.ttbr;
        // C s_12_8: const #1s : i64
        let s_12_8: i64 = 1;
        // C s_12_9: cast zx s_12_8 -> i
        let s_12_9: i128 = (i128::try_from(s_12_8).unwrap());
        // C s_12_10: const #46s : i
        let s_12_10: i128 = 46;
        // C s_12_11: add s_12_10 s_12_9
        let s_12_11: i128 = (s_12_10 + s_12_9);
        // D s_12_12: bit-extract s_12_7 s_12_6 s_12_11
        let s_12_12: Bits = (Bits::new(
            ((s_12_7) >> (s_12_6)).value(),
            u16::try_from(s_12_11).unwrap(),
        ));
        // D s_12_13: cast reint s_12_12 -> u47
        let s_12_13: u64 = (s_12_12.value() as u64);
        // C s_12_14: const #1s : i
        let s_12_14: i128 = 1;
        // D s_12_15: read-var tablebase:u56
        let s_12_15: u64 = fn_state.tablebase;
        // D s_12_16: cast zx s_12_15 -> bv
        let s_12_16: Bits = Bits::new(s_12_15 as u128, 56u16);
        // D s_12_17: cast zx s_12_13 -> bv
        let s_12_17: Bits = Bits::new(s_12_13 as u128, 47u16);
        // C s_12_18: const #46s : i
        let s_12_18: i128 = 46;
        // C s_12_19: const #1u : u64
        let s_12_19: u64 = 1;
        // C s_12_20: cast zx s_12_19 -> bv
        let s_12_20: Bits = Bits::new(s_12_19 as u128, 64u16);
        // C s_12_21: lsl s_12_20 s_12_18
        let s_12_21: Bits = s_12_20 << s_12_18;
        // C s_12_22: sub s_12_21 s_12_20
        let s_12_22: Bits = ((s_12_21) - (s_12_20));
        // D s_12_23: and s_12_17 s_12_22
        let s_12_23: Bits = ((s_12_17) & (s_12_22));
        // D s_12_24: lsl s_12_23 s_12_14
        let s_12_24: Bits = s_12_23 << s_12_14;
        // C s_12_25: lsl s_12_22 s_12_14
        let s_12_25: Bits = s_12_22 << s_12_14;
        // C s_12_26: cmpl s_12_25
        let s_12_26: Bits = !s_12_25;
        // D s_12_27: and s_12_16 s_12_26
        let s_12_27: Bits = ((s_12_16) & (s_12_26));
        // D s_12_28: or s_12_27 s_12_24
        let s_12_28: Bits = ((s_12_27) | (s_12_24));
        // D s_12_29: cast reint s_12_28 -> u56
        let s_12_29: u64 = (s_12_28.value() as u64);
        // D s_12_30: write-var tablebase <= s_12_29
        fn_state.tablebase = s_12_29;
        // N s_12_31: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_13_0: read-var tsize:i
        let s_13_0: i128 = fn_state.tsize;
        // C s_13_1: const #1s : i
        let s_13_1: i128 = 1;
        // D s_13_2: call _shl_int_general(s_13_1, s_13_0)
        let s_13_2: i128 = u_shl_int_general(state, tracer, s_13_1, s_13_0);
        // D s_13_3: read-var tablebase:u56
        let s_13_3: u64 = fn_state.tablebase;
        // D s_13_4: cast zx s_13_3 -> bv
        let s_13_4: Bits = Bits::new(s_13_3 as u128, 56u16);
        // D s_13_5: call Align_bits(s_13_4, s_13_2)
        let s_13_5: Bits = Align_bits(state, tracer, s_13_4, s_13_2);
        // D s_13_6: cast reint s_13_5 -> u56
        let s_13_6: u64 = (s_13_5.value() as u64);
        // N s_13_7: return s_13_6
        return s_13_6;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_14_0: const #6s : i
        let s_14_0: i128 = 6;
        // D s_14_1: read-var tsize:i
        let s_14_1: i128 = fn_state.tsize;
        // D s_14_2: cmp-gt s_14_1 s_14_0
        let s_14_2: bool = ((s_14_1) > (s_14_0));
        // D s_14_3: select s_14_2 s_14_1 s_14_0
        let s_14_3: i128 = if s_14_2 { s_14_1 } else { s_14_0 };
        // D s_14_4: write-var tsize <= s_14_3
        fn_state.tsize = s_14_3;
        // D s_14_5: read-var ttbr:bv
        let s_14_5: Bits = fn_state.ttbr;
        // D s_14_6: size-of s_14_5
        let s_14_6: u16 = s_14_5.length();
        // D s_14_7: cast zx s_14_6 -> i
        let s_14_7: i128 = (i128::try_from(s_14_6).unwrap());
        // C s_14_8: const #5s : i
        let s_14_8: i128 = 5;
        // D s_14_9: cmp-lt s_14_8 s_14_7
        let s_14_9: bool = ((s_14_8) < (s_14_7));
        // N s_14_10: assert s_14_9
        let s_14_10: () = assert!(s_14_9);
        // D s_14_11: read-var ttbr:bv
        let s_14_11: Bits = fn_state.ttbr;
        // D s_14_12: size-of s_14_11
        let s_14_12: u16 = s_14_11.length();
        // D s_14_13: cast zx s_14_12 -> i
        let s_14_13: i128 = (i128::try_from(s_14_12).unwrap());
        // C s_14_14: const #47s : i
        let s_14_14: i128 = 47;
        // D s_14_15: cmp-lt s_14_14 s_14_13
        let s_14_15: bool = ((s_14_14) < (s_14_13));
        // N s_14_16: assert s_14_15
        let s_14_16: () = assert!(s_14_15);
        // C s_14_17: const #2s : i
        let s_14_17: i128 = 2;
        // D s_14_18: read-var ttbr:bv
        let s_14_18: Bits = fn_state.ttbr;
        // C s_14_19: const #1s : i64
        let s_14_19: i64 = 1;
        // C s_14_20: cast zx s_14_19 -> i
        let s_14_20: i128 = (i128::try_from(s_14_19).unwrap());
        // C s_14_21: const #3s : i
        let s_14_21: i128 = 3;
        // C s_14_22: add s_14_21 s_14_20
        let s_14_22: i128 = (s_14_21 + s_14_20);
        // D s_14_23: bit-extract s_14_18 s_14_17 s_14_22
        let s_14_23: Bits = (Bits::new(
            ((s_14_18) >> (s_14_17)).value(),
            u16::try_from(s_14_22).unwrap(),
        ));
        // D s_14_24: cast reint s_14_23 -> u8
        let s_14_24: u8 = (s_14_23.value() as u8);
        // C s_14_25: const #6s : i
        let s_14_25: i128 = 6;
        // D s_14_26: read-var ttbr:bv
        let s_14_26: Bits = fn_state.ttbr;
        // C s_14_27: const #1s : i64
        let s_14_27: i64 = 1;
        // C s_14_28: cast zx s_14_27 -> i
        let s_14_28: i128 = (i128::try_from(s_14_27).unwrap());
        // C s_14_29: const #41s : i
        let s_14_29: i128 = 41;
        // C s_14_30: add s_14_29 s_14_28
        let s_14_30: i128 = (s_14_29 + s_14_28);
        // D s_14_31: bit-extract s_14_26 s_14_25 s_14_30
        let s_14_31: Bits = (Bits::new(
            ((s_14_26) >> (s_14_25)).value(),
            u16::try_from(s_14_30).unwrap(),
        ));
        // D s_14_32: cast reint s_14_31 -> u42
        let s_14_32: u64 = (s_14_31.value() as u64);
        // D s_14_33: cast zx s_14_24 -> bv
        let s_14_33: Bits = Bits::new(s_14_24 as u128, 4u16);
        // D s_14_34: cast zx s_14_32 -> bv
        let s_14_34: Bits = Bits::new(s_14_32 as u128, 42u16);
        // D s_14_35: cast reint s_14_33 -> u128
        let s_14_35: u128 = (s_14_33.value() as u128);
        // D s_14_36: size-of s_14_33
        let s_14_36: u16 = s_14_33.length();
        // D s_14_37: cast reint s_14_34 -> u128
        let s_14_37: u128 = (s_14_34.value() as u128);
        // D s_14_38: size-of s_14_34
        let s_14_38: u16 = s_14_34.length();
        // D s_14_39: lsl s_14_35 s_14_38
        let s_14_39: u128 = s_14_35 << s_14_38;
        // D s_14_40: or s_14_39 s_14_37
        let s_14_40: u128 = ((s_14_39) | (s_14_37));
        // D s_14_41: add s_14_36 s_14_38
        let s_14_41: u16 = (s_14_36 + s_14_38);
        // D s_14_42: create-bits s_14_40 s_14_41
        let s_14_42: Bits = Bits::new(s_14_40, s_14_41);
        // D s_14_43: cast reint s_14_42 -> u46
        let s_14_43: u64 = (s_14_42.value() as u64);
        // C s_14_44: const #6s : i
        let s_14_44: i128 = 6;
        // D s_14_45: read-var tablebase:u56
        let s_14_45: u64 = fn_state.tablebase;
        // D s_14_46: cast zx s_14_45 -> bv
        let s_14_46: Bits = Bits::new(s_14_45 as u128, 56u16);
        // D s_14_47: cast zx s_14_43 -> bv
        let s_14_47: Bits = Bits::new(s_14_43 as u128, 46u16);
        // C s_14_48: const #45s : i
        let s_14_48: i128 = 45;
        // C s_14_49: const #1u : u64
        let s_14_49: u64 = 1;
        // C s_14_50: cast zx s_14_49 -> bv
        let s_14_50: Bits = Bits::new(s_14_49 as u128, 64u16);
        // C s_14_51: lsl s_14_50 s_14_48
        let s_14_51: Bits = s_14_50 << s_14_48;
        // C s_14_52: sub s_14_51 s_14_50
        let s_14_52: Bits = ((s_14_51) - (s_14_50));
        // D s_14_53: and s_14_47 s_14_52
        let s_14_53: Bits = ((s_14_47) & (s_14_52));
        // D s_14_54: lsl s_14_53 s_14_44
        let s_14_54: Bits = s_14_53 << s_14_44;
        // C s_14_55: lsl s_14_52 s_14_44
        let s_14_55: Bits = s_14_52 << s_14_44;
        // C s_14_56: cmpl s_14_55
        let s_14_56: Bits = !s_14_55;
        // D s_14_57: and s_14_46 s_14_56
        let s_14_57: Bits = ((s_14_46) & (s_14_56));
        // D s_14_58: or s_14_57 s_14_54
        let s_14_58: Bits = ((s_14_57) | (s_14_54));
        // D s_14_59: cast reint s_14_58 -> u56
        let s_14_59: u64 = (s_14_58.value() as u64);
        // D s_14_60: write-var tablebase <= s_14_59
        fn_state.tablebase = s_14_59;
        // N s_14_61: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#19089 <= s_15_0
        fn_state.gs_19089 = s_15_0;
        // N s_15_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_16_0: read-var walkparams.14:struct
        let s_16_0: u8 = fn_state.walkparams._14;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 3u16);
        // C s_16_2: const #6u : u8
        let s_16_2: u8 = 6;
        // C s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 3u16);
        // D s_16_4: cmp-eq s_16_1 s_16_3
        let s_16_4: bool = ((s_16_1) == (s_16_3));
        // D s_16_5: write-var gs#19088 <= s_16_4
        fn_state.gs_19088 = s_16_4;
        // N s_16_6: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_17_0: read-var walkparams.26:struct
        let s_17_0: u32 = fn_state.walkparams._26;
        // C s_17_1: const #2u : u32
        let s_17_1: u32 = 2;
        // D s_17_2: cmp-eq s_17_0 s_17_1
        let s_17_2: bool = ((s_17_0) == (s_17_1));
        // D s_17_3: write-var gs#19087 <= s_17_2
        fn_state.gs_19087 = s_17_2;
        // N s_17_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_18_0: const #5s : i
        let s_18_0: i128 = 5;
        // D s_18_1: read-var tsize:i
        let s_18_1: i128 = fn_state.tsize;
        // D s_18_2: cmp-gt s_18_1 s_18_0
        let s_18_2: bool = ((s_18_1) > (s_18_0));
        // D s_18_3: select s_18_2 s_18_1 s_18_0
        let s_18_3: i128 = if s_18_2 { s_18_1 } else { s_18_0 };
        // D s_18_4: write-var tsize <= s_18_3
        fn_state.tsize = s_18_3;
        // D s_18_5: read-var paspace:u32
        let s_18_5: u32 = fn_state.paspace;
        // C s_18_6: const #1u : u32
        let s_18_6: u32 = 1;
        // D s_18_7: cmp-eq s_18_5 s_18_6
        let s_18_7: bool = ((s_18_5) == (s_18_6));
        // N s_18_8: branch s_18_7 b20 b19
        if s_18_7 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_19_0: read-var ttbr:bv
        let s_19_0: Bits = fn_state.ttbr;
        // D s_19_1: size-of s_19_0
        let s_19_1: u16 = s_19_0.length();
        // D s_19_2: cast zx s_19_1 -> i
        let s_19_2: i128 = (i128::try_from(s_19_1).unwrap());
        // C s_19_3: const #87s : i
        let s_19_3: i128 = 87;
        // D s_19_4: cmp-lt s_19_3 s_19_2
        let s_19_4: bool = ((s_19_3) < (s_19_2));
        // N s_19_5: assert s_19_4
        let s_19_5: () = assert!(s_19_4);
        // C s_19_6: const #80s : i
        let s_19_6: i128 = 80;
        // D s_19_7: read-var ttbr:bv
        let s_19_7: Bits = fn_state.ttbr;
        // C s_19_8: const #1s : i64
        let s_19_8: i64 = 1;
        // C s_19_9: cast zx s_19_8 -> i
        let s_19_9: i128 = (i128::try_from(s_19_8).unwrap());
        // C s_19_10: const #7s : i
        let s_19_10: i128 = 7;
        // C s_19_11: add s_19_10 s_19_9
        let s_19_11: i128 = (s_19_10 + s_19_9);
        // D s_19_12: bit-extract s_19_7 s_19_6 s_19_11
        let s_19_12: Bits = (Bits::new(
            ((s_19_7) >> (s_19_6)).value(),
            u16::try_from(s_19_11).unwrap(),
        ));
        // D s_19_13: cast reint s_19_12 -> u8
        let s_19_13: u8 = (s_19_12.value() as u8);
        // C s_19_14: const #5s : i
        let s_19_14: i128 = 5;
        // D s_19_15: read-var ttbr:bv
        let s_19_15: Bits = fn_state.ttbr;
        // C s_19_16: const #1s : i64
        let s_19_16: i64 = 1;
        // C s_19_17: cast zx s_19_16 -> i
        let s_19_17: i128 = (i128::try_from(s_19_16).unwrap());
        // C s_19_18: const #42s : i
        let s_19_18: i128 = 42;
        // C s_19_19: add s_19_18 s_19_17
        let s_19_19: i128 = (s_19_18 + s_19_17);
        // D s_19_20: bit-extract s_19_15 s_19_14 s_19_19
        let s_19_20: Bits = (Bits::new(
            ((s_19_15) >> (s_19_14)).value(),
            u16::try_from(s_19_19).unwrap(),
        ));
        // D s_19_21: cast reint s_19_20 -> u43
        let s_19_21: u64 = (s_19_20.value() as u64);
        // D s_19_22: cast zx s_19_13 -> bv
        let s_19_22: Bits = Bits::new(s_19_13 as u128, 8u16);
        // D s_19_23: cast zx s_19_21 -> bv
        let s_19_23: Bits = Bits::new(s_19_21 as u128, 43u16);
        // D s_19_24: cast reint s_19_22 -> u128
        let s_19_24: u128 = (s_19_22.value() as u128);
        // D s_19_25: size-of s_19_22
        let s_19_25: u16 = s_19_22.length();
        // D s_19_26: cast reint s_19_23 -> u128
        let s_19_26: u128 = (s_19_23.value() as u128);
        // D s_19_27: size-of s_19_23
        let s_19_27: u16 = s_19_23.length();
        // D s_19_28: lsl s_19_24 s_19_27
        let s_19_28: u128 = s_19_24 << s_19_27;
        // D s_19_29: or s_19_28 s_19_26
        let s_19_29: u128 = ((s_19_28) | (s_19_26));
        // D s_19_30: add s_19_25 s_19_27
        let s_19_30: u16 = (s_19_25 + s_19_27);
        // D s_19_31: create-bits s_19_29 s_19_30
        let s_19_31: Bits = Bits::new(s_19_29, s_19_30);
        // D s_19_32: cast reint s_19_31 -> u51
        let s_19_32: u64 = (s_19_31.value() as u64);
        // C s_19_33: const #5s : i
        let s_19_33: i128 = 5;
        // D s_19_34: read-var tablebase:u56
        let s_19_34: u64 = fn_state.tablebase;
        // D s_19_35: cast zx s_19_34 -> bv
        let s_19_35: Bits = Bits::new(s_19_34 as u128, 56u16);
        // D s_19_36: cast zx s_19_32 -> bv
        let s_19_36: Bits = Bits::new(s_19_32 as u128, 51u16);
        // C s_19_37: const #50s : i
        let s_19_37: i128 = 50;
        // C s_19_38: const #1u : u64
        let s_19_38: u64 = 1;
        // C s_19_39: cast zx s_19_38 -> bv
        let s_19_39: Bits = Bits::new(s_19_38 as u128, 64u16);
        // C s_19_40: lsl s_19_39 s_19_37
        let s_19_40: Bits = s_19_39 << s_19_37;
        // C s_19_41: sub s_19_40 s_19_39
        let s_19_41: Bits = ((s_19_40) - (s_19_39));
        // D s_19_42: and s_19_36 s_19_41
        let s_19_42: Bits = ((s_19_36) & (s_19_41));
        // D s_19_43: lsl s_19_42 s_19_33
        let s_19_43: Bits = s_19_42 << s_19_33;
        // C s_19_44: lsl s_19_41 s_19_33
        let s_19_44: Bits = s_19_41 << s_19_33;
        // C s_19_45: cmpl s_19_44
        let s_19_45: Bits = !s_19_44;
        // D s_19_46: and s_19_35 s_19_45
        let s_19_46: Bits = ((s_19_35) & (s_19_45));
        // D s_19_47: or s_19_46 s_19_43
        let s_19_47: Bits = ((s_19_46) | (s_19_43));
        // D s_19_48: cast reint s_19_47 -> u56
        let s_19_48: u64 = (s_19_47.value() as u64);
        // D s_19_49: write-var tablebase <= s_19_48
        fn_state.tablebase = s_19_48;
        // N s_19_50: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_20_0: read-var ttbr:bv
        let s_20_0: Bits = fn_state.ttbr;
        // D s_20_1: size-of s_20_0
        let s_20_1: u16 = s_20_0.length();
        // D s_20_2: cast zx s_20_1 -> i
        let s_20_2: i128 = (i128::try_from(s_20_1).unwrap());
        // C s_20_3: const #55s : i
        let s_20_3: i128 = 55;
        // D s_20_4: cmp-lt s_20_3 s_20_2
        let s_20_4: bool = ((s_20_3) < (s_20_2));
        // N s_20_5: assert s_20_4
        let s_20_5: () = assert!(s_20_4);
        // C s_20_6: const #5s : i
        let s_20_6: i128 = 5;
        // D s_20_7: read-var ttbr:bv
        let s_20_7: Bits = fn_state.ttbr;
        // C s_20_8: const #1s : i64
        let s_20_8: i64 = 1;
        // C s_20_9: cast zx s_20_8 -> i
        let s_20_9: i128 = (i128::try_from(s_20_8).unwrap());
        // C s_20_10: const #50s : i
        let s_20_10: i128 = 50;
        // C s_20_11: add s_20_10 s_20_9
        let s_20_11: i128 = (s_20_10 + s_20_9);
        // D s_20_12: bit-extract s_20_7 s_20_6 s_20_11
        let s_20_12: Bits = (Bits::new(
            ((s_20_7) >> (s_20_6)).value(),
            u16::try_from(s_20_11).unwrap(),
        ));
        // D s_20_13: cast reint s_20_12 -> u51
        let s_20_13: u64 = (s_20_12.value() as u64);
        // C s_20_14: const #5s : i
        let s_20_14: i128 = 5;
        // D s_20_15: read-var tablebase:u56
        let s_20_15: u64 = fn_state.tablebase;
        // D s_20_16: cast zx s_20_15 -> bv
        let s_20_16: Bits = Bits::new(s_20_15 as u128, 56u16);
        // D s_20_17: cast zx s_20_13 -> bv
        let s_20_17: Bits = Bits::new(s_20_13 as u128, 51u16);
        // C s_20_18: const #50s : i
        let s_20_18: i128 = 50;
        // C s_20_19: const #1u : u64
        let s_20_19: u64 = 1;
        // C s_20_20: cast zx s_20_19 -> bv
        let s_20_20: Bits = Bits::new(s_20_19 as u128, 64u16);
        // C s_20_21: lsl s_20_20 s_20_18
        let s_20_21: Bits = s_20_20 << s_20_18;
        // C s_20_22: sub s_20_21 s_20_20
        let s_20_22: Bits = ((s_20_21) - (s_20_20));
        // D s_20_23: and s_20_17 s_20_22
        let s_20_23: Bits = ((s_20_17) & (s_20_22));
        // D s_20_24: lsl s_20_23 s_20_14
        let s_20_24: Bits = s_20_23 << s_20_14;
        // C s_20_25: lsl s_20_22 s_20_14
        let s_20_25: Bits = s_20_22 << s_20_14;
        // C s_20_26: cmpl s_20_25
        let s_20_26: Bits = !s_20_25;
        // D s_20_27: and s_20_16 s_20_26
        let s_20_27: Bits = ((s_20_16) & (s_20_26));
        // D s_20_28: or s_20_27 s_20_24
        let s_20_28: Bits = ((s_20_27) | (s_20_24));
        // D s_20_29: cast reint s_20_28 -> u56
        let s_20_29: u64 = (s_20_28.value() as u64);
        // D s_20_30: write-var tablebase <= s_20_29
        fn_state.tablebase = s_20_29;
        // N s_20_31: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_21_0: read-var walkparams.2:struct
        let s_21_0: bool = fn_state.walkparams._2;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 1u16);
        // C s_21_2: const #1u : u8
        let s_21_2: bool = true;
        // C s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 1u16);
        // D s_21_4: cmp-eq s_21_1 s_21_3
        let s_21_4: bool = ((s_21_1) == (s_21_3));
        // D s_21_5: write-var gs#19085 <= s_21_4
        fn_state.gs_19085 = s_21_4;
        // N s_21_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_22_0: const #4s : i64
        let s_22_0: i64 = 4;
        // D s_22_1: write-var ga#14277 <= s_22_0
        fn_state.ga_14277 = s_22_0;
        // N s_22_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
