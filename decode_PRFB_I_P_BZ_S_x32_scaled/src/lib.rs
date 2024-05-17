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
use HaveSVE::*;
use CurrentVL_read::*;
use execute_PRFB_I_P_BZ_S_x32_scaled::*;
use common::*;
pub fn decode_PRFB_I_P_BZ_S_x32_scaled<T: Tracer>(
    state: &mut State,
    tracer: &T,
    xs: bool,
    Zm: u8,
    msz: u8,
    Pg: u8,
    Rn: u8,
    prfop: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        level: i64,
        VL: i64,
        pref_hint: u32,
        stream: bool,
        n: i64,
        offs_unsigned: bool,
        g: i64,
        xs: bool,
        Zm: u8,
        msz: u8,
        Pg: u8,
        Rn: u8,
        prfop: u8,
    }
    let fn_state = FunctionState {
        xs,
        Zm,
        msz,
        Pg,
        Rn,
        prfop,
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
        // S s_0_1: call CurrentVL_read(s_0_0)
        let s_0_1: i64 = CurrentVL_read(state, tracer, s_0_0);
        // D s_0_2: write-var VL <= s_0_1
        fn_state.VL = s_0_1;
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call HaveSVE(s_0_3)
        let s_0_4: bool = HaveSVE(state, tracer, s_0_3);
        // S s_0_5: not s_0_4
        let s_0_5: bool = !s_0_4;
        // N s_0_6: branch s_0_5 b15 b1
        if s_0_5 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var Pg:u8
        let s_1_0: u8 = fn_state.Pg;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 3u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // D s_1_4: write-var g <= s_1_3
        fn_state.g = s_1_3;
        // D s_1_5: read-var Rn:u8
        let s_1_5: u8 = fn_state.Rn;
        // D s_1_6: cast zx s_1_5 -> bv
        let s_1_6: Bits = Bits::new(s_1_5 as u128, 5u16);
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (s_1_6.value() as i128);
        // D s_1_8: cast reint s_1_7 -> i64
        let s_1_8: i64 = (s_1_7 as i64);
        // D s_1_9: write-var n <= s_1_8
        fn_state.n = s_1_8;
        // D s_1_10: read-var Zm:u8
        let s_1_10: u8 = fn_state.Zm;
        // D s_1_11: cast zx s_1_10 -> bv
        let s_1_11: Bits = Bits::new(s_1_10 as u128, 5u16);
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (s_1_11.value() as i128);
        // D s_1_13: cast reint s_1_12 -> i64
        let s_1_13: i64 = (s_1_12 as i64);
        // D s_1_14: write-var m <= s_1_13
        fn_state.m = s_1_13;
        // C s_1_15: const #1s : i
        let s_1_15: i128 = 1;
        // D s_1_16: read-var prfop:u8
        let s_1_16: u8 = fn_state.prfop;
        // D s_1_17: cast zx s_1_16 -> bv
        let s_1_17: Bits = Bits::new(s_1_16 as u128, 4u16);
        // C s_1_18: const #1s : i64
        let s_1_18: i64 = 1;
        // C s_1_19: cast zx s_1_18 -> i
        let s_1_19: i128 = (i128::try_from(s_1_18).unwrap());
        // C s_1_20: const #1s : i
        let s_1_20: i128 = 1;
        // C s_1_21: add s_1_20 s_1_19
        let s_1_21: i128 = (s_1_20 + s_1_19);
        // D s_1_22: bit-extract s_1_17 s_1_15 s_1_21
        let s_1_22: Bits = (Bits::new(
            ((s_1_17) >> (s_1_15)).value(),
            u16::try_from(s_1_21).unwrap(),
        ));
        // D s_1_23: cast reint s_1_22 -> u8
        let s_1_23: u8 = (s_1_22.value() as u8);
        // D s_1_24: cast zx s_1_23 -> bv
        let s_1_24: Bits = Bits::new(s_1_23 as u128, 2u16);
        // D s_1_25: cast zx s_1_24 -> i
        let s_1_25: i128 = (s_1_24.value() as i128);
        // D s_1_26: cast reint s_1_25 -> i64
        let s_1_26: i64 = (s_1_25 as i64);
        // D s_1_27: write-var level <= s_1_26
        fn_state.level = s_1_26;
        // C s_1_28: const #0s : i
        let s_1_28: i128 = 0;
        // D s_1_29: read-var prfop:u8
        let s_1_29: u8 = fn_state.prfop;
        // D s_1_30: cast zx s_1_29 -> bv
        let s_1_30: Bits = Bits::new(s_1_29 as u128, 4u16);
        // C s_1_31: const #1u : u64
        let s_1_31: u64 = 1;
        // D s_1_32: bit-extract s_1_30 s_1_28 s_1_31
        let s_1_32: Bits = (Bits::new(
            ((s_1_30) >> (s_1_28)).value(),
            u16::try_from(s_1_31).unwrap(),
        ));
        // D s_1_33: cast reint s_1_32 -> u8
        let s_1_33: bool = ((s_1_32.value()) != 0);
        // C s_1_34: const #0s : i
        let s_1_34: i128 = 0;
        // C s_1_35: const #0u : u64
        let s_1_35: u64 = 0;
        // D s_1_36: cast zx s_1_33 -> u64
        let s_1_36: u64 = (s_1_33 as u64);
        // C s_1_37: const #1u : u64
        let s_1_37: u64 = 1;
        // D s_1_38: and s_1_36 s_1_37
        let s_1_38: u64 = ((s_1_36) & (s_1_37));
        // D s_1_39: cmp-eq s_1_38 s_1_37
        let s_1_39: bool = ((s_1_38) == (s_1_37));
        // D s_1_40: lsl s_1_36 s_1_34
        let s_1_40: u64 = s_1_36 << s_1_34;
        // D s_1_41: or s_1_35 s_1_40
        let s_1_41: u64 = ((s_1_35) | (s_1_40));
        // D s_1_42: cmpl s_1_40
        let s_1_42: u64 = !s_1_40;
        // D s_1_43: and s_1_35 s_1_42
        let s_1_43: u64 = ((s_1_35) & (s_1_42));
        // D s_1_44: select s_1_39 s_1_41 s_1_43
        let s_1_44: u64 = if s_1_39 { s_1_41 } else { s_1_43 };
        // D s_1_45: cast trunc s_1_44 -> u8
        let s_1_45: bool = ((s_1_44) != 0);
        // D s_1_46: cast zx s_1_45 -> bv
        let s_1_46: Bits = Bits::new(s_1_45 as u128, 1u16);
        // C s_1_47: const #1u : u8
        let s_1_47: bool = true;
        // C s_1_48: cast zx s_1_47 -> bv
        let s_1_48: Bits = Bits::new(s_1_47 as u128, 1u16);
        // D s_1_49: cmp-eq s_1_46 s_1_48
        let s_1_49: bool = ((s_1_46) == (s_1_48));
        // D s_1_50: write-var stream <= s_1_49
        fn_state.stream = s_1_49;
        // C s_1_51: const #3s : i
        let s_1_51: i128 = 3;
        // D s_1_52: read-var prfop:u8
        let s_1_52: u8 = fn_state.prfop;
        // D s_1_53: cast zx s_1_52 -> bv
        let s_1_53: Bits = Bits::new(s_1_52 as u128, 4u16);
        // C s_1_54: const #1u : u64
        let s_1_54: u64 = 1;
        // D s_1_55: bit-extract s_1_53 s_1_51 s_1_54
        let s_1_55: Bits = (Bits::new(
            ((s_1_53) >> (s_1_51)).value(),
            u16::try_from(s_1_54).unwrap(),
        ));
        // D s_1_56: cast reint s_1_55 -> u8
        let s_1_56: bool = ((s_1_55.value()) != 0);
        // C s_1_57: const #0s : i
        let s_1_57: i128 = 0;
        // C s_1_58: const #0u : u64
        let s_1_58: u64 = 0;
        // D s_1_59: cast zx s_1_56 -> u64
        let s_1_59: u64 = (s_1_56 as u64);
        // C s_1_60: const #1u : u64
        let s_1_60: u64 = 1;
        // D s_1_61: and s_1_59 s_1_60
        let s_1_61: u64 = ((s_1_59) & (s_1_60));
        // D s_1_62: cmp-eq s_1_61 s_1_60
        let s_1_62: bool = ((s_1_61) == (s_1_60));
        // D s_1_63: lsl s_1_59 s_1_57
        let s_1_63: u64 = s_1_59 << s_1_57;
        // D s_1_64: or s_1_58 s_1_63
        let s_1_64: u64 = ((s_1_58) | (s_1_63));
        // D s_1_65: cmpl s_1_63
        let s_1_65: u64 = !s_1_63;
        // D s_1_66: and s_1_58 s_1_65
        let s_1_66: u64 = ((s_1_58) & (s_1_65));
        // D s_1_67: select s_1_62 s_1_64 s_1_66
        let s_1_67: u64 = if s_1_62 { s_1_64 } else { s_1_66 };
        // D s_1_68: cast trunc s_1_67 -> u8
        let s_1_68: bool = ((s_1_67) != 0);
        // D s_1_69: cast zx s_1_68 -> bv
        let s_1_69: Bits = Bits::new(s_1_68 as u128, 1u16);
        // C s_1_70: const #0u : u8
        let s_1_70: bool = false;
        // C s_1_71: cast zx s_1_70 -> bv
        let s_1_71: Bits = Bits::new(s_1_70 as u128, 1u16);
        // D s_1_72: cmp-eq s_1_69 s_1_71
        let s_1_72: bool = ((s_1_69) == (s_1_71));
        // N s_1_73: branch s_1_72 b14 b2
        if s_1_72 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #1u : u32
        let s_2_0: u32 = 1;
        // D s_2_1: write-var pref_hint <= s_2_0
        fn_state.pref_hint = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var xs:u8
        let s_3_0: bool = fn_state.xs;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 1u16);
        // C s_3_2: const #0u : u8
        let s_3_2: bool = false;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // D s_3_5: write-var offs_unsigned <= s_3_4
        fn_state.offs_unsigned = s_3_4;
        // D s_3_6: read-var VL:i64
        let s_3_6: i64 = fn_state.VL;
        // C s_3_7: const #128s : i
        let s_3_7: i128 = 128;
        // D s_3_8: cast zx s_3_6 -> i
        let s_3_8: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_9: cmp-eq s_3_8 s_3_7
        let s_3_9: bool = ((s_3_8) == (s_3_7));
        // D s_3_10: not s_3_9
        let s_3_10: bool = !s_3_9;
        // N s_3_11: branch s_3_10 b5 b4
        if s_3_10 {
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
        // C s_4_0: const #128s : i64
        let s_4_0: i64 = 128;
        // C s_4_1: const #32s : i64
        let s_4_1: i64 = 32;
        // D s_4_2: read-var g:i64
        let s_4_2: i64 = fn_state.g;
        // D s_4_3: read-var level:i64
        let s_4_3: i64 = fn_state.level;
        // D s_4_4: read-var m:i64
        let s_4_4: i64 = fn_state.m;
        // D s_4_5: read-var n:i64
        let s_4_5: i64 = fn_state.n;
        // C s_4_6: const #32s : i64
        let s_4_6: i64 = 32;
        // D s_4_7: read-var offs_unsigned:u8
        let s_4_7: bool = fn_state.offs_unsigned;
        // D s_4_8: read-var pref_hint:u32
        let s_4_8: u32 = fn_state.pref_hint;
        // C s_4_9: const #0s : i64
        let s_4_9: i64 = 0;
        // D s_4_10: read-var stream:u8
        let s_4_10: bool = fn_state.stream;
        // D s_4_11: call execute_PRFB_I_P_BZ_S_x32_scaled(s_4_0, s_4_1, s_4_2, s_4_3, s_4_4, s_4_5, s_4_6, s_4_7, s_4_8, s_4_9, s_4_10)
        let s_4_11: () = execute_PRFB_I_P_BZ_S_x32_scaled(
            state,
            tracer,
            s_4_0,
            s_4_1,
            s_4_2,
            s_4_3,
            s_4_4,
            s_4_5,
            s_4_6,
            s_4_7,
            s_4_8,
            s_4_9,
            s_4_10,
        );
        // N s_4_12: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var VL:i64
        let s_5_0: i64 = fn_state.VL;
        // C s_5_1: const #256s : i
        let s_5_1: i128 = 256;
        // D s_5_2: cast zx s_5_0 -> i
        let s_5_2: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_3: cmp-eq s_5_2 s_5_1
        let s_5_3: bool = ((s_5_2) == (s_5_1));
        // D s_5_4: not s_5_3
        let s_5_4: bool = !s_5_3;
        // N s_5_5: branch s_5_4 b7 b6
        if s_5_4 {
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
        // C s_6_0: const #256s : i64
        let s_6_0: i64 = 256;
        // C s_6_1: const #32s : i64
        let s_6_1: i64 = 32;
        // D s_6_2: read-var g:i64
        let s_6_2: i64 = fn_state.g;
        // D s_6_3: read-var level:i64
        let s_6_3: i64 = fn_state.level;
        // D s_6_4: read-var m:i64
        let s_6_4: i64 = fn_state.m;
        // D s_6_5: read-var n:i64
        let s_6_5: i64 = fn_state.n;
        // C s_6_6: const #32s : i64
        let s_6_6: i64 = 32;
        // D s_6_7: read-var offs_unsigned:u8
        let s_6_7: bool = fn_state.offs_unsigned;
        // D s_6_8: read-var pref_hint:u32
        let s_6_8: u32 = fn_state.pref_hint;
        // C s_6_9: const #0s : i64
        let s_6_9: i64 = 0;
        // D s_6_10: read-var stream:u8
        let s_6_10: bool = fn_state.stream;
        // D s_6_11: call execute_PRFB_I_P_BZ_S_x32_scaled(s_6_0, s_6_1, s_6_2, s_6_3, s_6_4, s_6_5, s_6_6, s_6_7, s_6_8, s_6_9, s_6_10)
        let s_6_11: () = execute_PRFB_I_P_BZ_S_x32_scaled(
            state,
            tracer,
            s_6_0,
            s_6_1,
            s_6_2,
            s_6_3,
            s_6_4,
            s_6_5,
            s_6_6,
            s_6_7,
            s_6_8,
            s_6_9,
            s_6_10,
        );
        // N s_6_12: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var VL:i64
        let s_7_0: i64 = fn_state.VL;
        // C s_7_1: const #512s : i
        let s_7_1: i128 = 512;
        // D s_7_2: cast zx s_7_0 -> i
        let s_7_2: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_3: cmp-eq s_7_2 s_7_1
        let s_7_3: bool = ((s_7_2) == (s_7_1));
        // D s_7_4: not s_7_3
        let s_7_4: bool = !s_7_3;
        // N s_7_5: branch s_7_4 b9 b8
        if s_7_4 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #512s : i64
        let s_8_0: i64 = 512;
        // C s_8_1: const #32s : i64
        let s_8_1: i64 = 32;
        // D s_8_2: read-var g:i64
        let s_8_2: i64 = fn_state.g;
        // D s_8_3: read-var level:i64
        let s_8_3: i64 = fn_state.level;
        // D s_8_4: read-var m:i64
        let s_8_4: i64 = fn_state.m;
        // D s_8_5: read-var n:i64
        let s_8_5: i64 = fn_state.n;
        // C s_8_6: const #32s : i64
        let s_8_6: i64 = 32;
        // D s_8_7: read-var offs_unsigned:u8
        let s_8_7: bool = fn_state.offs_unsigned;
        // D s_8_8: read-var pref_hint:u32
        let s_8_8: u32 = fn_state.pref_hint;
        // C s_8_9: const #0s : i64
        let s_8_9: i64 = 0;
        // D s_8_10: read-var stream:u8
        let s_8_10: bool = fn_state.stream;
        // D s_8_11: call execute_PRFB_I_P_BZ_S_x32_scaled(s_8_0, s_8_1, s_8_2, s_8_3, s_8_4, s_8_5, s_8_6, s_8_7, s_8_8, s_8_9, s_8_10)
        let s_8_11: () = execute_PRFB_I_P_BZ_S_x32_scaled(
            state,
            tracer,
            s_8_0,
            s_8_1,
            s_8_2,
            s_8_3,
            s_8_4,
            s_8_5,
            s_8_6,
            s_8_7,
            s_8_8,
            s_8_9,
            s_8_10,
        );
        // N s_8_12: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var VL:i64
        let s_9_0: i64 = fn_state.VL;
        // C s_9_1: const #1024s : i
        let s_9_1: i128 = 1024;
        // D s_9_2: cast zx s_9_0 -> i
        let s_9_2: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_3: cmp-eq s_9_2 s_9_1
        let s_9_3: bool = ((s_9_2) == (s_9_1));
        // D s_9_4: not s_9_3
        let s_9_4: bool = !s_9_3;
        // N s_9_5: branch s_9_4 b11 b10
        if s_9_4 {
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
        // C s_10_0: const #1024s : i64
        let s_10_0: i64 = 1024;
        // C s_10_1: const #32s : i64
        let s_10_1: i64 = 32;
        // D s_10_2: read-var g:i64
        let s_10_2: i64 = fn_state.g;
        // D s_10_3: read-var level:i64
        let s_10_3: i64 = fn_state.level;
        // D s_10_4: read-var m:i64
        let s_10_4: i64 = fn_state.m;
        // D s_10_5: read-var n:i64
        let s_10_5: i64 = fn_state.n;
        // C s_10_6: const #32s : i64
        let s_10_6: i64 = 32;
        // D s_10_7: read-var offs_unsigned:u8
        let s_10_7: bool = fn_state.offs_unsigned;
        // D s_10_8: read-var pref_hint:u32
        let s_10_8: u32 = fn_state.pref_hint;
        // C s_10_9: const #0s : i64
        let s_10_9: i64 = 0;
        // D s_10_10: read-var stream:u8
        let s_10_10: bool = fn_state.stream;
        // D s_10_11: call execute_PRFB_I_P_BZ_S_x32_scaled(s_10_0, s_10_1, s_10_2, s_10_3, s_10_4, s_10_5, s_10_6, s_10_7, s_10_8, s_10_9, s_10_10)
        let s_10_11: () = execute_PRFB_I_P_BZ_S_x32_scaled(
            state,
            tracer,
            s_10_0,
            s_10_1,
            s_10_2,
            s_10_3,
            s_10_4,
            s_10_5,
            s_10_6,
            s_10_7,
            s_10_8,
            s_10_9,
            s_10_10,
        );
        // N s_10_12: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var VL:i64
        let s_11_0: i64 = fn_state.VL;
        // C s_11_1: const #2048s : i
        let s_11_1: i128 = 2048;
        // D s_11_2: cast zx s_11_0 -> i
        let s_11_2: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_3: cmp-eq s_11_2 s_11_1
        let s_11_3: bool = ((s_11_2) == (s_11_1));
        // D s_11_4: not s_11_3
        let s_11_4: bool = !s_11_3;
        // N s_11_5: branch s_11_4 b13 b12
        if s_11_4 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #2048s : i64
        let s_12_0: i64 = 2048;
        // C s_12_1: const #32s : i64
        let s_12_1: i64 = 32;
        // D s_12_2: read-var g:i64
        let s_12_2: i64 = fn_state.g;
        // D s_12_3: read-var level:i64
        let s_12_3: i64 = fn_state.level;
        // D s_12_4: read-var m:i64
        let s_12_4: i64 = fn_state.m;
        // D s_12_5: read-var n:i64
        let s_12_5: i64 = fn_state.n;
        // C s_12_6: const #32s : i64
        let s_12_6: i64 = 32;
        // D s_12_7: read-var offs_unsigned:u8
        let s_12_7: bool = fn_state.offs_unsigned;
        // D s_12_8: read-var pref_hint:u32
        let s_12_8: u32 = fn_state.pref_hint;
        // C s_12_9: const #0s : i64
        let s_12_9: i64 = 0;
        // D s_12_10: read-var stream:u8
        let s_12_10: bool = fn_state.stream;
        // D s_12_11: call execute_PRFB_I_P_BZ_S_x32_scaled(s_12_0, s_12_1, s_12_2, s_12_3, s_12_4, s_12_5, s_12_6, s_12_7, s_12_8, s_12_9, s_12_10)
        let s_12_11: () = execute_PRFB_I_P_BZ_S_x32_scaled(
            state,
            tracer,
            s_12_0,
            s_12_1,
            s_12_2,
            s_12_3,
            s_12_4,
            s_12_5,
            s_12_6,
            s_12_7,
            s_12_8,
            s_12_9,
            s_12_10,
        );
        // N s_12_12: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u32
        let s_14_0: u32 = 0;
        // D s_14_1: write-var pref_hint <= s_14_0
        fn_state.pref_hint = s_14_0;
        // N s_14_2: jump b3
        return block_3(state, tracer, fn_state);
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
}
