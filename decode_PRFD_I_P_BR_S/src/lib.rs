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
use execute_PRFD_I_P_BR_S::*;
use HaveSME::*;
use common::*;
pub fn decode_PRFD_I_P_BR_S<T: Tracer>(
    state: &mut State,
    tracer: &T,
    msz: u8,
    Rm: u8,
    Pg: u8,
    Rn: u8,
    prfop: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        level: i64,
        gs_226278: bool,
        VL: i64,
        pref_hint: u32,
        stream: bool,
        n: i64,
        g: i64,
        msz: u8,
        Rm: u8,
        Pg: u8,
        Rn: u8,
        prfop: u8,
    }
    let fn_state = FunctionState {
        msz,
        Rm,
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
        // N s_0_6: branch s_0_5 b20 b1
        if s_0_5 {
            return block_20(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#226278 <= s_1_0
        fn_state.gs_226278 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#226278:u8
        let s_2_0: bool = fn_state.gs_226278;
        // N s_2_1: branch s_2_0 b19 b3
        if s_2_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var Rm:u8
        let s_3_0: u8 = fn_state.Rm;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 5u16);
        // C s_3_2: const #31u : u8
        let s_3_2: u8 = 31;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 5u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b18 b4
        if s_3_4 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var Pg:u8
        let s_4_0: u8 = fn_state.Pg;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 3u16);
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (s_4_1.value() as i128);
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // D s_4_4: write-var g <= s_4_3
        fn_state.g = s_4_3;
        // D s_4_5: read-var Rn:u8
        let s_4_5: u8 = fn_state.Rn;
        // D s_4_6: cast zx s_4_5 -> bv
        let s_4_6: Bits = Bits::new(s_4_5 as u128, 5u16);
        // D s_4_7: cast zx s_4_6 -> i
        let s_4_7: i128 = (s_4_6.value() as i128);
        // D s_4_8: cast reint s_4_7 -> i64
        let s_4_8: i64 = (s_4_7 as i64);
        // D s_4_9: write-var n <= s_4_8
        fn_state.n = s_4_8;
        // D s_4_10: read-var Rm:u8
        let s_4_10: u8 = fn_state.Rm;
        // D s_4_11: cast zx s_4_10 -> bv
        let s_4_11: Bits = Bits::new(s_4_10 as u128, 5u16);
        // D s_4_12: cast zx s_4_11 -> i
        let s_4_12: i128 = (s_4_11.value() as i128);
        // D s_4_13: cast reint s_4_12 -> i64
        let s_4_13: i64 = (s_4_12 as i64);
        // D s_4_14: write-var m <= s_4_13
        fn_state.m = s_4_13;
        // C s_4_15: const #1s : i
        let s_4_15: i128 = 1;
        // D s_4_16: read-var prfop:u8
        let s_4_16: u8 = fn_state.prfop;
        // D s_4_17: cast zx s_4_16 -> bv
        let s_4_17: Bits = Bits::new(s_4_16 as u128, 4u16);
        // C s_4_18: const #1s : i64
        let s_4_18: i64 = 1;
        // C s_4_19: cast zx s_4_18 -> i
        let s_4_19: i128 = (i128::try_from(s_4_18).unwrap());
        // C s_4_20: const #1s : i
        let s_4_20: i128 = 1;
        // C s_4_21: add s_4_20 s_4_19
        let s_4_21: i128 = (s_4_20 + s_4_19);
        // D s_4_22: bit-extract s_4_17 s_4_15 s_4_21
        let s_4_22: Bits = (Bits::new(
            ((s_4_17) >> (s_4_15)).value(),
            u16::try_from(s_4_21).unwrap(),
        ));
        // D s_4_23: cast reint s_4_22 -> u8
        let s_4_23: u8 = (s_4_22.value() as u8);
        // D s_4_24: cast zx s_4_23 -> bv
        let s_4_24: Bits = Bits::new(s_4_23 as u128, 2u16);
        // D s_4_25: cast zx s_4_24 -> i
        let s_4_25: i128 = (s_4_24.value() as i128);
        // D s_4_26: cast reint s_4_25 -> i64
        let s_4_26: i64 = (s_4_25 as i64);
        // D s_4_27: write-var level <= s_4_26
        fn_state.level = s_4_26;
        // C s_4_28: const #0s : i
        let s_4_28: i128 = 0;
        // D s_4_29: read-var prfop:u8
        let s_4_29: u8 = fn_state.prfop;
        // D s_4_30: cast zx s_4_29 -> bv
        let s_4_30: Bits = Bits::new(s_4_29 as u128, 4u16);
        // C s_4_31: const #1u : u64
        let s_4_31: u64 = 1;
        // D s_4_32: bit-extract s_4_30 s_4_28 s_4_31
        let s_4_32: Bits = (Bits::new(
            ((s_4_30) >> (s_4_28)).value(),
            u16::try_from(s_4_31).unwrap(),
        ));
        // D s_4_33: cast reint s_4_32 -> u8
        let s_4_33: bool = ((s_4_32.value()) != 0);
        // C s_4_34: const #0s : i
        let s_4_34: i128 = 0;
        // C s_4_35: const #0u : u64
        let s_4_35: u64 = 0;
        // D s_4_36: cast zx s_4_33 -> u64
        let s_4_36: u64 = (s_4_33 as u64);
        // C s_4_37: const #1u : u64
        let s_4_37: u64 = 1;
        // D s_4_38: and s_4_36 s_4_37
        let s_4_38: u64 = ((s_4_36) & (s_4_37));
        // D s_4_39: cmp-eq s_4_38 s_4_37
        let s_4_39: bool = ((s_4_38) == (s_4_37));
        // D s_4_40: lsl s_4_36 s_4_34
        let s_4_40: u64 = s_4_36 << s_4_34;
        // D s_4_41: or s_4_35 s_4_40
        let s_4_41: u64 = ((s_4_35) | (s_4_40));
        // D s_4_42: cmpl s_4_40
        let s_4_42: u64 = !s_4_40;
        // D s_4_43: and s_4_35 s_4_42
        let s_4_43: u64 = ((s_4_35) & (s_4_42));
        // D s_4_44: select s_4_39 s_4_41 s_4_43
        let s_4_44: u64 = if s_4_39 { s_4_41 } else { s_4_43 };
        // D s_4_45: cast trunc s_4_44 -> u8
        let s_4_45: bool = ((s_4_44) != 0);
        // D s_4_46: cast zx s_4_45 -> bv
        let s_4_46: Bits = Bits::new(s_4_45 as u128, 1u16);
        // C s_4_47: const #1u : u8
        let s_4_47: bool = true;
        // C s_4_48: cast zx s_4_47 -> bv
        let s_4_48: Bits = Bits::new(s_4_47 as u128, 1u16);
        // D s_4_49: cmp-eq s_4_46 s_4_48
        let s_4_49: bool = ((s_4_46) == (s_4_48));
        // D s_4_50: write-var stream <= s_4_49
        fn_state.stream = s_4_49;
        // C s_4_51: const #3s : i
        let s_4_51: i128 = 3;
        // D s_4_52: read-var prfop:u8
        let s_4_52: u8 = fn_state.prfop;
        // D s_4_53: cast zx s_4_52 -> bv
        let s_4_53: Bits = Bits::new(s_4_52 as u128, 4u16);
        // C s_4_54: const #1u : u64
        let s_4_54: u64 = 1;
        // D s_4_55: bit-extract s_4_53 s_4_51 s_4_54
        let s_4_55: Bits = (Bits::new(
            ((s_4_53) >> (s_4_51)).value(),
            u16::try_from(s_4_54).unwrap(),
        ));
        // D s_4_56: cast reint s_4_55 -> u8
        let s_4_56: bool = ((s_4_55.value()) != 0);
        // C s_4_57: const #0s : i
        let s_4_57: i128 = 0;
        // C s_4_58: const #0u : u64
        let s_4_58: u64 = 0;
        // D s_4_59: cast zx s_4_56 -> u64
        let s_4_59: u64 = (s_4_56 as u64);
        // C s_4_60: const #1u : u64
        let s_4_60: u64 = 1;
        // D s_4_61: and s_4_59 s_4_60
        let s_4_61: u64 = ((s_4_59) & (s_4_60));
        // D s_4_62: cmp-eq s_4_61 s_4_60
        let s_4_62: bool = ((s_4_61) == (s_4_60));
        // D s_4_63: lsl s_4_59 s_4_57
        let s_4_63: u64 = s_4_59 << s_4_57;
        // D s_4_64: or s_4_58 s_4_63
        let s_4_64: u64 = ((s_4_58) | (s_4_63));
        // D s_4_65: cmpl s_4_63
        let s_4_65: u64 = !s_4_63;
        // D s_4_66: and s_4_58 s_4_65
        let s_4_66: u64 = ((s_4_58) & (s_4_65));
        // D s_4_67: select s_4_62 s_4_64 s_4_66
        let s_4_67: u64 = if s_4_62 { s_4_64 } else { s_4_66 };
        // D s_4_68: cast trunc s_4_67 -> u8
        let s_4_68: bool = ((s_4_67) != 0);
        // D s_4_69: cast zx s_4_68 -> bv
        let s_4_69: Bits = Bits::new(s_4_68 as u128, 1u16);
        // C s_4_70: const #0u : u8
        let s_4_70: bool = false;
        // C s_4_71: cast zx s_4_70 -> bv
        let s_4_71: Bits = Bits::new(s_4_70 as u128, 1u16);
        // D s_4_72: cmp-eq s_4_69 s_4_71
        let s_4_72: bool = ((s_4_69) == (s_4_71));
        // N s_4_73: branch s_4_72 b17 b5
        if s_4_72 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #1u : u32
        let s_5_0: u32 = 1;
        // D s_5_1: write-var pref_hint <= s_5_0
        fn_state.pref_hint = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var VL:i64
        let s_6_0: i64 = fn_state.VL;
        // C s_6_1: const #128s : i
        let s_6_1: i128 = 128;
        // D s_6_2: cast zx s_6_0 -> i
        let s_6_2: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_3: cmp-eq s_6_2 s_6_1
        let s_6_3: bool = ((s_6_2) == (s_6_1));
        // D s_6_4: not s_6_3
        let s_6_4: bool = !s_6_3;
        // N s_6_5: branch s_6_4 b8 b7
        if s_6_4 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #128s : i64
        let s_7_0: i64 = 128;
        // C s_7_1: const #64s : i64
        let s_7_1: i64 = 64;
        // D s_7_2: read-var g:i64
        let s_7_2: i64 = fn_state.g;
        // D s_7_3: read-var level:i64
        let s_7_3: i64 = fn_state.level;
        // D s_7_4: read-var m:i64
        let s_7_4: i64 = fn_state.m;
        // D s_7_5: read-var n:i64
        let s_7_5: i64 = fn_state.n;
        // D s_7_6: read-var pref_hint:u32
        let s_7_6: u32 = fn_state.pref_hint;
        // C s_7_7: const #3s : i64
        let s_7_7: i64 = 3;
        // D s_7_8: read-var stream:u8
        let s_7_8: bool = fn_state.stream;
        // D s_7_9: call execute_PRFD_I_P_BR_S(s_7_0, s_7_1, s_7_2, s_7_3, s_7_4, s_7_5, s_7_6, s_7_7, s_7_8)
        let s_7_9: () = execute_PRFD_I_P_BR_S(
            state,
            tracer,
            s_7_0,
            s_7_1,
            s_7_2,
            s_7_3,
            s_7_4,
            s_7_5,
            s_7_6,
            s_7_7,
            s_7_8,
        );
        // N s_7_10: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var VL:i64
        let s_8_0: i64 = fn_state.VL;
        // C s_8_1: const #256s : i
        let s_8_1: i128 = 256;
        // D s_8_2: cast zx s_8_0 -> i
        let s_8_2: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_3: cmp-eq s_8_2 s_8_1
        let s_8_3: bool = ((s_8_2) == (s_8_1));
        // D s_8_4: not s_8_3
        let s_8_4: bool = !s_8_3;
        // N s_8_5: branch s_8_4 b10 b9
        if s_8_4 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #256s : i64
        let s_9_0: i64 = 256;
        // C s_9_1: const #64s : i64
        let s_9_1: i64 = 64;
        // D s_9_2: read-var g:i64
        let s_9_2: i64 = fn_state.g;
        // D s_9_3: read-var level:i64
        let s_9_3: i64 = fn_state.level;
        // D s_9_4: read-var m:i64
        let s_9_4: i64 = fn_state.m;
        // D s_9_5: read-var n:i64
        let s_9_5: i64 = fn_state.n;
        // D s_9_6: read-var pref_hint:u32
        let s_9_6: u32 = fn_state.pref_hint;
        // C s_9_7: const #3s : i64
        let s_9_7: i64 = 3;
        // D s_9_8: read-var stream:u8
        let s_9_8: bool = fn_state.stream;
        // D s_9_9: call execute_PRFD_I_P_BR_S(s_9_0, s_9_1, s_9_2, s_9_3, s_9_4, s_9_5, s_9_6, s_9_7, s_9_8)
        let s_9_9: () = execute_PRFD_I_P_BR_S(
            state,
            tracer,
            s_9_0,
            s_9_1,
            s_9_2,
            s_9_3,
            s_9_4,
            s_9_5,
            s_9_6,
            s_9_7,
            s_9_8,
        );
        // N s_9_10: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var VL:i64
        let s_10_0: i64 = fn_state.VL;
        // C s_10_1: const #512s : i
        let s_10_1: i128 = 512;
        // D s_10_2: cast zx s_10_0 -> i
        let s_10_2: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_3: cmp-eq s_10_2 s_10_1
        let s_10_3: bool = ((s_10_2) == (s_10_1));
        // D s_10_4: not s_10_3
        let s_10_4: bool = !s_10_3;
        // N s_10_5: branch s_10_4 b12 b11
        if s_10_4 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #512s : i64
        let s_11_0: i64 = 512;
        // C s_11_1: const #64s : i64
        let s_11_1: i64 = 64;
        // D s_11_2: read-var g:i64
        let s_11_2: i64 = fn_state.g;
        // D s_11_3: read-var level:i64
        let s_11_3: i64 = fn_state.level;
        // D s_11_4: read-var m:i64
        let s_11_4: i64 = fn_state.m;
        // D s_11_5: read-var n:i64
        let s_11_5: i64 = fn_state.n;
        // D s_11_6: read-var pref_hint:u32
        let s_11_6: u32 = fn_state.pref_hint;
        // C s_11_7: const #3s : i64
        let s_11_7: i64 = 3;
        // D s_11_8: read-var stream:u8
        let s_11_8: bool = fn_state.stream;
        // D s_11_9: call execute_PRFD_I_P_BR_S(s_11_0, s_11_1, s_11_2, s_11_3, s_11_4, s_11_5, s_11_6, s_11_7, s_11_8)
        let s_11_9: () = execute_PRFD_I_P_BR_S(
            state,
            tracer,
            s_11_0,
            s_11_1,
            s_11_2,
            s_11_3,
            s_11_4,
            s_11_5,
            s_11_6,
            s_11_7,
            s_11_8,
        );
        // N s_11_10: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var VL:i64
        let s_12_0: i64 = fn_state.VL;
        // C s_12_1: const #1024s : i
        let s_12_1: i128 = 1024;
        // D s_12_2: cast zx s_12_0 -> i
        let s_12_2: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_3: cmp-eq s_12_2 s_12_1
        let s_12_3: bool = ((s_12_2) == (s_12_1));
        // D s_12_4: not s_12_3
        let s_12_4: bool = !s_12_3;
        // N s_12_5: branch s_12_4 b14 b13
        if s_12_4 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1024s : i64
        let s_13_0: i64 = 1024;
        // C s_13_1: const #64s : i64
        let s_13_1: i64 = 64;
        // D s_13_2: read-var g:i64
        let s_13_2: i64 = fn_state.g;
        // D s_13_3: read-var level:i64
        let s_13_3: i64 = fn_state.level;
        // D s_13_4: read-var m:i64
        let s_13_4: i64 = fn_state.m;
        // D s_13_5: read-var n:i64
        let s_13_5: i64 = fn_state.n;
        // D s_13_6: read-var pref_hint:u32
        let s_13_6: u32 = fn_state.pref_hint;
        // C s_13_7: const #3s : i64
        let s_13_7: i64 = 3;
        // D s_13_8: read-var stream:u8
        let s_13_8: bool = fn_state.stream;
        // D s_13_9: call execute_PRFD_I_P_BR_S(s_13_0, s_13_1, s_13_2, s_13_3, s_13_4, s_13_5, s_13_6, s_13_7, s_13_8)
        let s_13_9: () = execute_PRFD_I_P_BR_S(
            state,
            tracer,
            s_13_0,
            s_13_1,
            s_13_2,
            s_13_3,
            s_13_4,
            s_13_5,
            s_13_6,
            s_13_7,
            s_13_8,
        );
        // N s_13_10: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var VL:i64
        let s_14_0: i64 = fn_state.VL;
        // C s_14_1: const #2048s : i
        let s_14_1: i128 = 2048;
        // D s_14_2: cast zx s_14_0 -> i
        let s_14_2: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_3: cmp-eq s_14_2 s_14_1
        let s_14_3: bool = ((s_14_2) == (s_14_1));
        // D s_14_4: not s_14_3
        let s_14_4: bool = !s_14_3;
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
    ) -> () {
        // C s_15_0: const #2048s : i64
        let s_15_0: i64 = 2048;
        // C s_15_1: const #64s : i64
        let s_15_1: i64 = 64;
        // D s_15_2: read-var g:i64
        let s_15_2: i64 = fn_state.g;
        // D s_15_3: read-var level:i64
        let s_15_3: i64 = fn_state.level;
        // D s_15_4: read-var m:i64
        let s_15_4: i64 = fn_state.m;
        // D s_15_5: read-var n:i64
        let s_15_5: i64 = fn_state.n;
        // D s_15_6: read-var pref_hint:u32
        let s_15_6: u32 = fn_state.pref_hint;
        // C s_15_7: const #3s : i64
        let s_15_7: i64 = 3;
        // D s_15_8: read-var stream:u8
        let s_15_8: bool = fn_state.stream;
        // D s_15_9: call execute_PRFD_I_P_BR_S(s_15_0, s_15_1, s_15_2, s_15_3, s_15_4, s_15_5, s_15_6, s_15_7, s_15_8)
        let s_15_9: () = execute_PRFD_I_P_BR_S(
            state,
            tracer,
            s_15_0,
            s_15_1,
            s_15_2,
            s_15_3,
            s_15_4,
            s_15_5,
            s_15_6,
            s_15_7,
            s_15_8,
        );
        // N s_15_10: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0u : u32
        let s_17_0: u32 = 0;
        // D s_17_1: write-var pref_hint <= s_17_0
        fn_state.pref_hint = s_17_0;
        // N s_17_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: panic
        panic!("{:?}", ());
        // N s_18_1: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: panic
        panic!("{:?}", ());
        // N s_19_1: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call HaveSME(s_20_0)
        let s_20_1: bool = HaveSME(state, tracer, s_20_0);
        // S s_20_2: not s_20_1
        let s_20_2: bool = !s_20_1;
        // D s_20_3: write-var gs#226278 <= s_20_2
        fn_state.gs_226278 = s_20_2;
        // N s_20_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
