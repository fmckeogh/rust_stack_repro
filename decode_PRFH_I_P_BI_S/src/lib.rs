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
use HaveSME::*;
use CurrentVL_read::*;
use execute_PRFH_I_P_BI_S::*;
use common::*;
pub fn decode_PRFH_I_P_BI_S<T: Tracer>(
    state: &mut State,
    tracer: &T,
    imm6: u8,
    msz: u8,
    Pg: u8,
    Rn: u8,
    prfop: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        level: i64,
        VL: i64,
        pref_hint: u32,
        stream: bool,
        gs_225895: bool,
        n: i64,
        offset: i64,
        g: i64,
        imm6: u8,
        msz: u8,
        Pg: u8,
        Rn: u8,
        prfop: u8,
    }
    let fn_state = FunctionState {
        imm6,
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
        // N s_0_6: branch s_0_5 b18 b1
        if s_0_5 {
            return block_18(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#225895 <= s_1_0
        fn_state.gs_225895 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#225895:u8
        let s_2_0: bool = fn_state.gs_225895;
        // N s_2_1: branch s_2_0 b17 b3
        if s_2_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var Pg:u8
        let s_3_0: u8 = fn_state.Pg;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 3u16);
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // D s_3_4: write-var g <= s_3_3
        fn_state.g = s_3_3;
        // D s_3_5: read-var Rn:u8
        let s_3_5: u8 = fn_state.Rn;
        // D s_3_6: cast zx s_3_5 -> bv
        let s_3_6: Bits = Bits::new(s_3_5 as u128, 5u16);
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (s_3_6.value() as i128);
        // D s_3_8: cast reint s_3_7 -> i64
        let s_3_8: i64 = (s_3_7 as i64);
        // D s_3_9: write-var n <= s_3_8
        fn_state.n = s_3_8;
        // C s_3_10: const #1s : i
        let s_3_10: i128 = 1;
        // D s_3_11: read-var prfop:u8
        let s_3_11: u8 = fn_state.prfop;
        // D s_3_12: cast zx s_3_11 -> bv
        let s_3_12: Bits = Bits::new(s_3_11 as u128, 4u16);
        // C s_3_13: const #1s : i64
        let s_3_13: i64 = 1;
        // C s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // C s_3_15: const #1s : i
        let s_3_15: i128 = 1;
        // C s_3_16: add s_3_15 s_3_14
        let s_3_16: i128 = (s_3_15 + s_3_14);
        // D s_3_17: bit-extract s_3_12 s_3_10 s_3_16
        let s_3_17: Bits = (Bits::new(
            ((s_3_12) >> (s_3_10)).value(),
            u16::try_from(s_3_16).unwrap(),
        ));
        // D s_3_18: cast reint s_3_17 -> u8
        let s_3_18: u8 = (s_3_17.value() as u8);
        // D s_3_19: cast zx s_3_18 -> bv
        let s_3_19: Bits = Bits::new(s_3_18 as u128, 2u16);
        // D s_3_20: cast zx s_3_19 -> i
        let s_3_20: i128 = (s_3_19.value() as i128);
        // D s_3_21: cast reint s_3_20 -> i64
        let s_3_21: i64 = (s_3_20 as i64);
        // D s_3_22: write-var level <= s_3_21
        fn_state.level = s_3_21;
        // C s_3_23: const #0s : i
        let s_3_23: i128 = 0;
        // D s_3_24: read-var prfop:u8
        let s_3_24: u8 = fn_state.prfop;
        // D s_3_25: cast zx s_3_24 -> bv
        let s_3_25: Bits = Bits::new(s_3_24 as u128, 4u16);
        // C s_3_26: const #1u : u64
        let s_3_26: u64 = 1;
        // D s_3_27: bit-extract s_3_25 s_3_23 s_3_26
        let s_3_27: Bits = (Bits::new(
            ((s_3_25) >> (s_3_23)).value(),
            u16::try_from(s_3_26).unwrap(),
        ));
        // D s_3_28: cast reint s_3_27 -> u8
        let s_3_28: bool = ((s_3_27.value()) != 0);
        // C s_3_29: const #0s : i
        let s_3_29: i128 = 0;
        // C s_3_30: const #0u : u64
        let s_3_30: u64 = 0;
        // D s_3_31: cast zx s_3_28 -> u64
        let s_3_31: u64 = (s_3_28 as u64);
        // C s_3_32: const #1u : u64
        let s_3_32: u64 = 1;
        // D s_3_33: and s_3_31 s_3_32
        let s_3_33: u64 = ((s_3_31) & (s_3_32));
        // D s_3_34: cmp-eq s_3_33 s_3_32
        let s_3_34: bool = ((s_3_33) == (s_3_32));
        // D s_3_35: lsl s_3_31 s_3_29
        let s_3_35: u64 = s_3_31 << s_3_29;
        // D s_3_36: or s_3_30 s_3_35
        let s_3_36: u64 = ((s_3_30) | (s_3_35));
        // D s_3_37: cmpl s_3_35
        let s_3_37: u64 = !s_3_35;
        // D s_3_38: and s_3_30 s_3_37
        let s_3_38: u64 = ((s_3_30) & (s_3_37));
        // D s_3_39: select s_3_34 s_3_36 s_3_38
        let s_3_39: u64 = if s_3_34 { s_3_36 } else { s_3_38 };
        // D s_3_40: cast trunc s_3_39 -> u8
        let s_3_40: bool = ((s_3_39) != 0);
        // D s_3_41: cast zx s_3_40 -> bv
        let s_3_41: Bits = Bits::new(s_3_40 as u128, 1u16);
        // C s_3_42: const #1u : u8
        let s_3_42: bool = true;
        // C s_3_43: cast zx s_3_42 -> bv
        let s_3_43: Bits = Bits::new(s_3_42 as u128, 1u16);
        // D s_3_44: cmp-eq s_3_41 s_3_43
        let s_3_44: bool = ((s_3_41) == (s_3_43));
        // D s_3_45: write-var stream <= s_3_44
        fn_state.stream = s_3_44;
        // C s_3_46: const #3s : i
        let s_3_46: i128 = 3;
        // D s_3_47: read-var prfop:u8
        let s_3_47: u8 = fn_state.prfop;
        // D s_3_48: cast zx s_3_47 -> bv
        let s_3_48: Bits = Bits::new(s_3_47 as u128, 4u16);
        // C s_3_49: const #1u : u64
        let s_3_49: u64 = 1;
        // D s_3_50: bit-extract s_3_48 s_3_46 s_3_49
        let s_3_50: Bits = (Bits::new(
            ((s_3_48) >> (s_3_46)).value(),
            u16::try_from(s_3_49).unwrap(),
        ));
        // D s_3_51: cast reint s_3_50 -> u8
        let s_3_51: bool = ((s_3_50.value()) != 0);
        // C s_3_52: const #0s : i
        let s_3_52: i128 = 0;
        // C s_3_53: const #0u : u64
        let s_3_53: u64 = 0;
        // D s_3_54: cast zx s_3_51 -> u64
        let s_3_54: u64 = (s_3_51 as u64);
        // C s_3_55: const #1u : u64
        let s_3_55: u64 = 1;
        // D s_3_56: and s_3_54 s_3_55
        let s_3_56: u64 = ((s_3_54) & (s_3_55));
        // D s_3_57: cmp-eq s_3_56 s_3_55
        let s_3_57: bool = ((s_3_56) == (s_3_55));
        // D s_3_58: lsl s_3_54 s_3_52
        let s_3_58: u64 = s_3_54 << s_3_52;
        // D s_3_59: or s_3_53 s_3_58
        let s_3_59: u64 = ((s_3_53) | (s_3_58));
        // D s_3_60: cmpl s_3_58
        let s_3_60: u64 = !s_3_58;
        // D s_3_61: and s_3_53 s_3_60
        let s_3_61: u64 = ((s_3_53) & (s_3_60));
        // D s_3_62: select s_3_57 s_3_59 s_3_61
        let s_3_62: u64 = if s_3_57 { s_3_59 } else { s_3_61 };
        // D s_3_63: cast trunc s_3_62 -> u8
        let s_3_63: bool = ((s_3_62) != 0);
        // D s_3_64: cast zx s_3_63 -> bv
        let s_3_64: Bits = Bits::new(s_3_63 as u128, 1u16);
        // C s_3_65: const #0u : u8
        let s_3_65: bool = false;
        // C s_3_66: cast zx s_3_65 -> bv
        let s_3_66: Bits = Bits::new(s_3_65 as u128, 1u16);
        // D s_3_67: cmp-eq s_3_64 s_3_66
        let s_3_67: bool = ((s_3_64) == (s_3_66));
        // N s_3_68: branch s_3_67 b16 b4
        if s_3_67 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #1u : u32
        let s_4_0: u32 = 1;
        // D s_4_1: write-var pref_hint <= s_4_0
        fn_state.pref_hint = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var imm6:u8
        let s_5_0: u8 = fn_state.imm6;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 6u16);
        // D s_5_2: cast sx s_5_1 -> i
        let s_5_2: i128 = {
            let sign_bit = s_5_1.length() - 1;
            let mut result = s_5_1.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // D s_5_4: write-var offset <= s_5_3
        fn_state.offset = s_5_3;
        // D s_5_5: read-var VL:i64
        let s_5_5: i64 = fn_state.VL;
        // C s_5_6: const #128s : i
        let s_5_6: i128 = 128;
        // D s_5_7: cast zx s_5_5 -> i
        let s_5_7: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_8: cmp-eq s_5_7 s_5_6
        let s_5_8: bool = ((s_5_7) == (s_5_6));
        // D s_5_9: not s_5_8
        let s_5_9: bool = !s_5_8;
        // N s_5_10: branch s_5_9 b7 b6
        if s_5_9 {
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
        // C s_6_0: const #128s : i64
        let s_6_0: i64 = 128;
        // D s_6_1: read-var offset:i64
        let s_6_1: i64 = fn_state.offset;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // C s_6_3: const #16s : i64
        let s_6_3: i64 = 16;
        // D s_6_4: read-var g:i64
        let s_6_4: i64 = fn_state.g;
        // D s_6_5: read-var level:i64
        let s_6_5: i64 = fn_state.level;
        // D s_6_6: read-var n:i64
        let s_6_6: i64 = fn_state.n;
        // D s_6_7: read-var pref_hint:u32
        let s_6_7: u32 = fn_state.pref_hint;
        // C s_6_8: const #1s : i64
        let s_6_8: i64 = 1;
        // D s_6_9: read-var stream:u8
        let s_6_9: bool = fn_state.stream;
        // D s_6_10: call execute_PRFH_I_P_BI_S(s_6_0, s_6_3, s_6_4, s_6_5, s_6_6, s_6_2, s_6_7, s_6_8, s_6_9)
        let s_6_10: () = execute_PRFH_I_P_BI_S(
            state,
            tracer,
            s_6_0,
            s_6_3,
            s_6_4,
            s_6_5,
            s_6_6,
            s_6_2,
            s_6_7,
            s_6_8,
            s_6_9,
        );
        // N s_6_11: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var VL:i64
        let s_7_0: i64 = fn_state.VL;
        // C s_7_1: const #256s : i
        let s_7_1: i128 = 256;
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
        // C s_8_0: const #256s : i64
        let s_8_0: i64 = 256;
        // D s_8_1: read-var offset:i64
        let s_8_1: i64 = fn_state.offset;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // C s_8_3: const #16s : i64
        let s_8_3: i64 = 16;
        // D s_8_4: read-var g:i64
        let s_8_4: i64 = fn_state.g;
        // D s_8_5: read-var level:i64
        let s_8_5: i64 = fn_state.level;
        // D s_8_6: read-var n:i64
        let s_8_6: i64 = fn_state.n;
        // D s_8_7: read-var pref_hint:u32
        let s_8_7: u32 = fn_state.pref_hint;
        // C s_8_8: const #1s : i64
        let s_8_8: i64 = 1;
        // D s_8_9: read-var stream:u8
        let s_8_9: bool = fn_state.stream;
        // D s_8_10: call execute_PRFH_I_P_BI_S(s_8_0, s_8_3, s_8_4, s_8_5, s_8_6, s_8_2, s_8_7, s_8_8, s_8_9)
        let s_8_10: () = execute_PRFH_I_P_BI_S(
            state,
            tracer,
            s_8_0,
            s_8_3,
            s_8_4,
            s_8_5,
            s_8_6,
            s_8_2,
            s_8_7,
            s_8_8,
            s_8_9,
        );
        // N s_8_11: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var VL:i64
        let s_9_0: i64 = fn_state.VL;
        // C s_9_1: const #512s : i
        let s_9_1: i128 = 512;
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
        // C s_10_0: const #512s : i64
        let s_10_0: i64 = 512;
        // D s_10_1: read-var offset:i64
        let s_10_1: i64 = fn_state.offset;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // C s_10_3: const #16s : i64
        let s_10_3: i64 = 16;
        // D s_10_4: read-var g:i64
        let s_10_4: i64 = fn_state.g;
        // D s_10_5: read-var level:i64
        let s_10_5: i64 = fn_state.level;
        // D s_10_6: read-var n:i64
        let s_10_6: i64 = fn_state.n;
        // D s_10_7: read-var pref_hint:u32
        let s_10_7: u32 = fn_state.pref_hint;
        // C s_10_8: const #1s : i64
        let s_10_8: i64 = 1;
        // D s_10_9: read-var stream:u8
        let s_10_9: bool = fn_state.stream;
        // D s_10_10: call execute_PRFH_I_P_BI_S(s_10_0, s_10_3, s_10_4, s_10_5, s_10_6, s_10_2, s_10_7, s_10_8, s_10_9)
        let s_10_10: () = execute_PRFH_I_P_BI_S(
            state,
            tracer,
            s_10_0,
            s_10_3,
            s_10_4,
            s_10_5,
            s_10_6,
            s_10_2,
            s_10_7,
            s_10_8,
            s_10_9,
        );
        // N s_10_11: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var VL:i64
        let s_11_0: i64 = fn_state.VL;
        // C s_11_1: const #1024s : i
        let s_11_1: i128 = 1024;
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
        // C s_12_0: const #1024s : i64
        let s_12_0: i64 = 1024;
        // D s_12_1: read-var offset:i64
        let s_12_1: i64 = fn_state.offset;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // C s_12_3: const #16s : i64
        let s_12_3: i64 = 16;
        // D s_12_4: read-var g:i64
        let s_12_4: i64 = fn_state.g;
        // D s_12_5: read-var level:i64
        let s_12_5: i64 = fn_state.level;
        // D s_12_6: read-var n:i64
        let s_12_6: i64 = fn_state.n;
        // D s_12_7: read-var pref_hint:u32
        let s_12_7: u32 = fn_state.pref_hint;
        // C s_12_8: const #1s : i64
        let s_12_8: i64 = 1;
        // D s_12_9: read-var stream:u8
        let s_12_9: bool = fn_state.stream;
        // D s_12_10: call execute_PRFH_I_P_BI_S(s_12_0, s_12_3, s_12_4, s_12_5, s_12_6, s_12_2, s_12_7, s_12_8, s_12_9)
        let s_12_10: () = execute_PRFH_I_P_BI_S(
            state,
            tracer,
            s_12_0,
            s_12_3,
            s_12_4,
            s_12_5,
            s_12_6,
            s_12_2,
            s_12_7,
            s_12_8,
            s_12_9,
        );
        // N s_12_11: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var VL:i64
        let s_13_0: i64 = fn_state.VL;
        // C s_13_1: const #2048s : i
        let s_13_1: i128 = 2048;
        // D s_13_2: cast zx s_13_0 -> i
        let s_13_2: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_3: cmp-eq s_13_2 s_13_1
        let s_13_3: bool = ((s_13_2) == (s_13_1));
        // D s_13_4: not s_13_3
        let s_13_4: bool = !s_13_3;
        // N s_13_5: branch s_13_4 b15 b14
        if s_13_4 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #2048s : i64
        let s_14_0: i64 = 2048;
        // D s_14_1: read-var offset:i64
        let s_14_1: i64 = fn_state.offset;
        // D s_14_2: cast zx s_14_1 -> i
        let s_14_2: i128 = (i128::try_from(s_14_1).unwrap());
        // C s_14_3: const #16s : i64
        let s_14_3: i64 = 16;
        // D s_14_4: read-var g:i64
        let s_14_4: i64 = fn_state.g;
        // D s_14_5: read-var level:i64
        let s_14_5: i64 = fn_state.level;
        // D s_14_6: read-var n:i64
        let s_14_6: i64 = fn_state.n;
        // D s_14_7: read-var pref_hint:u32
        let s_14_7: u32 = fn_state.pref_hint;
        // C s_14_8: const #1s : i64
        let s_14_8: i64 = 1;
        // D s_14_9: read-var stream:u8
        let s_14_9: bool = fn_state.stream;
        // D s_14_10: call execute_PRFH_I_P_BI_S(s_14_0, s_14_3, s_14_4, s_14_5, s_14_6, s_14_2, s_14_7, s_14_8, s_14_9)
        let s_14_10: () = execute_PRFH_I_P_BI_S(
            state,
            tracer,
            s_14_0,
            s_14_3,
            s_14_4,
            s_14_5,
            s_14_6,
            s_14_2,
            s_14_7,
            s_14_8,
            s_14_9,
        );
        // N s_14_11: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u32
        let s_16_0: u32 = 0;
        // D s_16_1: write-var pref_hint <= s_16_0
        fn_state.pref_hint = s_16_0;
        // N s_16_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: panic
        panic!("{:?}", ());
        // N s_17_1: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call HaveSME(s_18_0)
        let s_18_1: bool = HaveSME(state, tracer, s_18_0);
        // S s_18_2: not s_18_1
        let s_18_2: bool = !s_18_1;
        // D s_18_3: write-var gs#225895 <= s_18_2
        fn_state.gs_225895 = s_18_2;
        // N s_18_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
