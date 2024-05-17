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
use execute_CDOT_Z_ZZZ__::*;
use CurrentVL_read::*;
use HaveSVE2::*;
use HaveSME::*;
use common::*;
pub fn decode_CDOT_Z_ZZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    size: u8,
    Zm: u8,
    rot: u8,
    Zn: u8,
    Zda: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        esize: i64,
        gs_216650: bool,
        n: i64,
        gs_216649: bool,
        sub_i: bool,
        VL: i64,
        sel_a: i64,
        sel_b: i64,
        da: i64,
        size: u8,
        Zm: u8,
        rot: u8,
        Zn: u8,
        Zda: u8,
    }
    let fn_state = FunctionState {
        size,
        Zm,
        rot,
        Zn,
        Zda,
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
        // S s_0_4: call HaveSVE2(s_0_3)
        let s_0_4: bool = HaveSVE2(state, tracer, s_0_3);
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
        // D s_1_1: write-var gs#216649 <= s_1_0
        fn_state.gs_216649 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#216649:u8
        let s_2_0: bool = fn_state.gs_216649;
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
        // D s_3_0: read-var size:u8
        let s_3_0: u8 = fn_state.size;
        // C s_3_1: const #1s : i
        let s_3_1: i128 = 1;
        // D s_3_2: cast zx s_3_0 -> bv
        let s_3_2: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_3: const #1s : i64
        let s_3_3: i64 = 1;
        // C s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // C s_3_5: const #0s : i
        let s_3_5: i128 = 0;
        // C s_3_6: add s_3_5 s_3_4
        let s_3_6: i128 = (s_3_5 + s_3_4);
        // D s_3_7: bit-extract s_3_2 s_3_1 s_3_6
        let s_3_7: Bits = (Bits::new(
            ((s_3_2) >> (s_3_1)).value(),
            u16::try_from(s_3_6).unwrap(),
        ));
        // D s_3_8: cast reint s_3_7 -> u8
        let s_3_8: bool = ((s_3_7.value()) != 0);
        // D s_3_9: cast zx s_3_8 -> bv
        let s_3_9: Bits = Bits::new(s_3_8 as u128, 1u16);
        // C s_3_10: const #0u : u8
        let s_3_10: bool = false;
        // C s_3_11: cast zx s_3_10 -> bv
        let s_3_11: Bits = Bits::new(s_3_10 as u128, 1u16);
        // D s_3_12: cmp-eq s_3_9 s_3_11
        let s_3_12: bool = ((s_3_9) == (s_3_11));
        // D s_3_13: not s_3_12
        let s_3_13: bool = !s_3_12;
        // N s_3_14: branch s_3_13 b18 b4
        if s_3_13 {
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
        // C s_4_0: const #1u : u8
        let s_4_0: bool = true;
        // D s_4_1: write-var gs#216650 <= s_4_0
        fn_state.gs_216650 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#216650:u8
        let s_5_0: bool = fn_state.gs_216650;
        // N s_5_1: branch s_5_0 b17 b6
        if s_5_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var size:u8
        let s_6_0: u8 = fn_state.size;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (s_6_1.value() as i128);
        // D s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // C s_6_4: const #8s : i64
        let s_6_4: i64 = 8;
        // D s_6_5: lsl s_6_4 s_6_3
        let s_6_5: i64 = s_6_4 << s_6_3;
        // D s_6_6: write-var esize <= s_6_5
        fn_state.esize = s_6_5;
        // D s_6_7: read-var Zn:u8
        let s_6_7: u8 = fn_state.Zn;
        // D s_6_8: cast zx s_6_7 -> bv
        let s_6_8: Bits = Bits::new(s_6_7 as u128, 5u16);
        // D s_6_9: cast zx s_6_8 -> i
        let s_6_9: i128 = (s_6_8.value() as i128);
        // D s_6_10: cast reint s_6_9 -> i64
        let s_6_10: i64 = (s_6_9 as i64);
        // D s_6_11: write-var n <= s_6_10
        fn_state.n = s_6_10;
        // D s_6_12: read-var Zm:u8
        let s_6_12: u8 = fn_state.Zm;
        // D s_6_13: cast zx s_6_12 -> bv
        let s_6_13: Bits = Bits::new(s_6_12 as u128, 5u16);
        // D s_6_14: cast zx s_6_13 -> i
        let s_6_14: i128 = (s_6_13.value() as i128);
        // D s_6_15: cast reint s_6_14 -> i64
        let s_6_15: i64 = (s_6_14 as i64);
        // D s_6_16: write-var m <= s_6_15
        fn_state.m = s_6_15;
        // D s_6_17: read-var Zda:u8
        let s_6_17: u8 = fn_state.Zda;
        // D s_6_18: cast zx s_6_17 -> bv
        let s_6_18: Bits = Bits::new(s_6_17 as u128, 5u16);
        // D s_6_19: cast zx s_6_18 -> i
        let s_6_19: i128 = (s_6_18.value() as i128);
        // D s_6_20: cast reint s_6_19 -> i64
        let s_6_20: i64 = (s_6_19 as i64);
        // D s_6_21: write-var da <= s_6_20
        fn_state.da = s_6_20;
        // C s_6_22: const #0s : i
        let s_6_22: i128 = 0;
        // D s_6_23: read-var rot:u8
        let s_6_23: u8 = fn_state.rot;
        // D s_6_24: cast zx s_6_23 -> bv
        let s_6_24: Bits = Bits::new(s_6_23 as u128, 2u16);
        // C s_6_25: const #1u : u64
        let s_6_25: u64 = 1;
        // D s_6_26: bit-extract s_6_24 s_6_22 s_6_25
        let s_6_26: Bits = (Bits::new(
            ((s_6_24) >> (s_6_22)).value(),
            u16::try_from(s_6_25).unwrap(),
        ));
        // D s_6_27: cast reint s_6_26 -> u8
        let s_6_27: bool = ((s_6_26.value()) != 0);
        // C s_6_28: const #0s : i
        let s_6_28: i128 = 0;
        // C s_6_29: const #0u : u64
        let s_6_29: u64 = 0;
        // D s_6_30: cast zx s_6_27 -> u64
        let s_6_30: u64 = (s_6_27 as u64);
        // C s_6_31: const #1u : u64
        let s_6_31: u64 = 1;
        // D s_6_32: and s_6_30 s_6_31
        let s_6_32: u64 = ((s_6_30) & (s_6_31));
        // D s_6_33: cmp-eq s_6_32 s_6_31
        let s_6_33: bool = ((s_6_32) == (s_6_31));
        // D s_6_34: lsl s_6_30 s_6_28
        let s_6_34: u64 = s_6_30 << s_6_28;
        // D s_6_35: or s_6_29 s_6_34
        let s_6_35: u64 = ((s_6_29) | (s_6_34));
        // D s_6_36: cmpl s_6_34
        let s_6_36: u64 = !s_6_34;
        // D s_6_37: and s_6_29 s_6_36
        let s_6_37: u64 = ((s_6_29) & (s_6_36));
        // D s_6_38: select s_6_33 s_6_35 s_6_37
        let s_6_38: u64 = if s_6_33 { s_6_35 } else { s_6_37 };
        // D s_6_39: cast trunc s_6_38 -> u8
        let s_6_39: bool = ((s_6_38) != 0);
        // D s_6_40: cast zx s_6_39 -> bv
        let s_6_40: Bits = Bits::new(s_6_39 as u128, 1u16);
        // D s_6_41: cast zx s_6_40 -> i
        let s_6_41: i128 = (s_6_40.value() as i128);
        // D s_6_42: cast reint s_6_41 -> i64
        let s_6_42: i64 = (s_6_41 as i64);
        // D s_6_43: write-var sel_a <= s_6_42
        fn_state.sel_a = s_6_42;
        // C s_6_44: const #0s : i
        let s_6_44: i128 = 0;
        // D s_6_45: read-var rot:u8
        let s_6_45: u8 = fn_state.rot;
        // D s_6_46: cast zx s_6_45 -> bv
        let s_6_46: Bits = Bits::new(s_6_45 as u128, 2u16);
        // C s_6_47: const #1u : u64
        let s_6_47: u64 = 1;
        // D s_6_48: bit-extract s_6_46 s_6_44 s_6_47
        let s_6_48: Bits = (Bits::new(
            ((s_6_46) >> (s_6_44)).value(),
            u16::try_from(s_6_47).unwrap(),
        ));
        // D s_6_49: cast reint s_6_48 -> u8
        let s_6_49: bool = ((s_6_48.value()) != 0);
        // C s_6_50: const #0s : i
        let s_6_50: i128 = 0;
        // C s_6_51: const #0u : u64
        let s_6_51: u64 = 0;
        // D s_6_52: cast zx s_6_49 -> u64
        let s_6_52: u64 = (s_6_49 as u64);
        // C s_6_53: const #1u : u64
        let s_6_53: u64 = 1;
        // D s_6_54: and s_6_52 s_6_53
        let s_6_54: u64 = ((s_6_52) & (s_6_53));
        // D s_6_55: cmp-eq s_6_54 s_6_53
        let s_6_55: bool = ((s_6_54) == (s_6_53));
        // D s_6_56: lsl s_6_52 s_6_50
        let s_6_56: u64 = s_6_52 << s_6_50;
        // D s_6_57: or s_6_51 s_6_56
        let s_6_57: u64 = ((s_6_51) | (s_6_56));
        // D s_6_58: cmpl s_6_56
        let s_6_58: u64 = !s_6_56;
        // D s_6_59: and s_6_51 s_6_58
        let s_6_59: u64 = ((s_6_51) & (s_6_58));
        // D s_6_60: select s_6_55 s_6_57 s_6_59
        let s_6_60: u64 = if s_6_55 { s_6_57 } else { s_6_59 };
        // D s_6_61: cast trunc s_6_60 -> u8
        let s_6_61: bool = ((s_6_60) != 0);
        // D s_6_62: cast zx s_6_61 -> bv
        let s_6_62: Bits = Bits::new(s_6_61 as u128, 1u16);
        // D s_6_63: not s_6_62
        let s_6_63: Bits = !s_6_62;
        // D s_6_64: cast reint s_6_63 -> u8
        let s_6_64: bool = ((s_6_63.value()) != 0);
        // D s_6_65: cast zx s_6_64 -> bv
        let s_6_65: Bits = Bits::new(s_6_64 as u128, 1u16);
        // D s_6_66: cast zx s_6_65 -> i
        let s_6_66: i128 = (s_6_65.value() as i128);
        // D s_6_67: cast reint s_6_66 -> i64
        let s_6_67: i64 = (s_6_66 as i64);
        // D s_6_68: write-var sel_b <= s_6_67
        fn_state.sel_b = s_6_67;
        // C s_6_69: const #0s : i
        let s_6_69: i128 = 0;
        // D s_6_70: read-var rot:u8
        let s_6_70: u8 = fn_state.rot;
        // D s_6_71: cast zx s_6_70 -> bv
        let s_6_71: Bits = Bits::new(s_6_70 as u128, 2u16);
        // C s_6_72: const #1u : u64
        let s_6_72: u64 = 1;
        // D s_6_73: bit-extract s_6_71 s_6_69 s_6_72
        let s_6_73: Bits = (Bits::new(
            ((s_6_71) >> (s_6_69)).value(),
            u16::try_from(s_6_72).unwrap(),
        ));
        // D s_6_74: cast reint s_6_73 -> u8
        let s_6_74: bool = ((s_6_73.value()) != 0);
        // C s_6_75: const #0s : i
        let s_6_75: i128 = 0;
        // C s_6_76: const #0u : u64
        let s_6_76: u64 = 0;
        // D s_6_77: cast zx s_6_74 -> u64
        let s_6_77: u64 = (s_6_74 as u64);
        // C s_6_78: const #1u : u64
        let s_6_78: u64 = 1;
        // D s_6_79: and s_6_77 s_6_78
        let s_6_79: u64 = ((s_6_77) & (s_6_78));
        // D s_6_80: cmp-eq s_6_79 s_6_78
        let s_6_80: bool = ((s_6_79) == (s_6_78));
        // D s_6_81: lsl s_6_77 s_6_75
        let s_6_81: u64 = s_6_77 << s_6_75;
        // D s_6_82: or s_6_76 s_6_81
        let s_6_82: u64 = ((s_6_76) | (s_6_81));
        // D s_6_83: cmpl s_6_81
        let s_6_83: u64 = !s_6_81;
        // D s_6_84: and s_6_76 s_6_83
        let s_6_84: u64 = ((s_6_76) & (s_6_83));
        // D s_6_85: select s_6_80 s_6_82 s_6_84
        let s_6_85: u64 = if s_6_80 { s_6_82 } else { s_6_84 };
        // D s_6_86: cast trunc s_6_85 -> u8
        let s_6_86: bool = ((s_6_85) != 0);
        // C s_6_87: const #1s : i
        let s_6_87: i128 = 1;
        // D s_6_88: read-var rot:u8
        let s_6_88: u8 = fn_state.rot;
        // D s_6_89: cast zx s_6_88 -> bv
        let s_6_89: Bits = Bits::new(s_6_88 as u128, 2u16);
        // C s_6_90: const #1u : u64
        let s_6_90: u64 = 1;
        // D s_6_91: bit-extract s_6_89 s_6_87 s_6_90
        let s_6_91: Bits = (Bits::new(
            ((s_6_89) >> (s_6_87)).value(),
            u16::try_from(s_6_90).unwrap(),
        ));
        // D s_6_92: cast reint s_6_91 -> u8
        let s_6_92: bool = ((s_6_91.value()) != 0);
        // C s_6_93: const #0s : i
        let s_6_93: i128 = 0;
        // C s_6_94: const #0u : u64
        let s_6_94: u64 = 0;
        // D s_6_95: cast zx s_6_92 -> u64
        let s_6_95: u64 = (s_6_92 as u64);
        // C s_6_96: const #1u : u64
        let s_6_96: u64 = 1;
        // D s_6_97: and s_6_95 s_6_96
        let s_6_97: u64 = ((s_6_95) & (s_6_96));
        // D s_6_98: cmp-eq s_6_97 s_6_96
        let s_6_98: bool = ((s_6_97) == (s_6_96));
        // D s_6_99: lsl s_6_95 s_6_93
        let s_6_99: u64 = s_6_95 << s_6_93;
        // D s_6_100: or s_6_94 s_6_99
        let s_6_100: u64 = ((s_6_94) | (s_6_99));
        // D s_6_101: cmpl s_6_99
        let s_6_101: u64 = !s_6_99;
        // D s_6_102: and s_6_94 s_6_101
        let s_6_102: u64 = ((s_6_94) & (s_6_101));
        // D s_6_103: select s_6_98 s_6_100 s_6_102
        let s_6_103: u64 = if s_6_98 { s_6_100 } else { s_6_102 };
        // D s_6_104: cast trunc s_6_103 -> u8
        let s_6_104: bool = ((s_6_103) != 0);
        // D s_6_105: cast zx s_6_86 -> bv
        let s_6_105: Bits = Bits::new(s_6_86 as u128, 1u16);
        // D s_6_106: cast zx s_6_104 -> bv
        let s_6_106: Bits = Bits::new(s_6_104 as u128, 1u16);
        // D s_6_107: cmp-eq s_6_105 s_6_106
        let s_6_107: bool = ((s_6_105) == (s_6_106));
        // D s_6_108: write-var sub_i <= s_6_107
        fn_state.sub_i = s_6_107;
        // D s_6_109: read-var VL:i64
        let s_6_109: i64 = fn_state.VL;
        // C s_6_110: const #128s : i
        let s_6_110: i128 = 128;
        // D s_6_111: cast zx s_6_109 -> i
        let s_6_111: i128 = (i128::try_from(s_6_109).unwrap());
        // D s_6_112: cmp-eq s_6_111 s_6_110
        let s_6_112: bool = ((s_6_111) == (s_6_110));
        // D s_6_113: not s_6_112
        let s_6_113: bool = !s_6_112;
        // N s_6_114: branch s_6_113 b8 b7
        if s_6_113 {
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
        // D s_7_1: read-var esize:i64
        let s_7_1: i64 = fn_state.esize;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: cast reint s_7_2 -> i64
        let s_7_3: i64 = (s_7_2 as i64);
        // D s_7_4: read-var da:i64
        let s_7_4: i64 = fn_state.da;
        // D s_7_5: read-var m:i64
        let s_7_5: i64 = fn_state.m;
        // D s_7_6: read-var n:i64
        let s_7_6: i64 = fn_state.n;
        // D s_7_7: read-var sel_a:i64
        let s_7_7: i64 = fn_state.sel_a;
        // D s_7_8: read-var sel_b:i64
        let s_7_8: i64 = fn_state.sel_b;
        // D s_7_9: read-var sub_i:u8
        let s_7_9: bool = fn_state.sub_i;
        // D s_7_10: call execute_CDOT_Z_ZZZ__(s_7_0, s_7_4, s_7_3, s_7_5, s_7_6, s_7_7, s_7_8, s_7_9)
        let s_7_10: () = execute_CDOT_Z_ZZZ__(
            state,
            tracer,
            s_7_0,
            s_7_4,
            s_7_3,
            s_7_5,
            s_7_6,
            s_7_7,
            s_7_8,
            s_7_9,
        );
        // N s_7_11: return
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
        // D s_9_1: read-var esize:i64
        let s_9_1: i64 = fn_state.esize;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // D s_9_4: read-var da:i64
        let s_9_4: i64 = fn_state.da;
        // D s_9_5: read-var m:i64
        let s_9_5: i64 = fn_state.m;
        // D s_9_6: read-var n:i64
        let s_9_6: i64 = fn_state.n;
        // D s_9_7: read-var sel_a:i64
        let s_9_7: i64 = fn_state.sel_a;
        // D s_9_8: read-var sel_b:i64
        let s_9_8: i64 = fn_state.sel_b;
        // D s_9_9: read-var sub_i:u8
        let s_9_9: bool = fn_state.sub_i;
        // D s_9_10: call execute_CDOT_Z_ZZZ__(s_9_0, s_9_4, s_9_3, s_9_5, s_9_6, s_9_7, s_9_8, s_9_9)
        let s_9_10: () = execute_CDOT_Z_ZZZ__(
            state,
            tracer,
            s_9_0,
            s_9_4,
            s_9_3,
            s_9_5,
            s_9_6,
            s_9_7,
            s_9_8,
            s_9_9,
        );
        // N s_9_11: return
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
        // D s_11_1: read-var esize:i64
        let s_11_1: i64 = fn_state.esize;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: cast reint s_11_2 -> i64
        let s_11_3: i64 = (s_11_2 as i64);
        // D s_11_4: read-var da:i64
        let s_11_4: i64 = fn_state.da;
        // D s_11_5: read-var m:i64
        let s_11_5: i64 = fn_state.m;
        // D s_11_6: read-var n:i64
        let s_11_6: i64 = fn_state.n;
        // D s_11_7: read-var sel_a:i64
        let s_11_7: i64 = fn_state.sel_a;
        // D s_11_8: read-var sel_b:i64
        let s_11_8: i64 = fn_state.sel_b;
        // D s_11_9: read-var sub_i:u8
        let s_11_9: bool = fn_state.sub_i;
        // D s_11_10: call execute_CDOT_Z_ZZZ__(s_11_0, s_11_4, s_11_3, s_11_5, s_11_6, s_11_7, s_11_8, s_11_9)
        let s_11_10: () = execute_CDOT_Z_ZZZ__(
            state,
            tracer,
            s_11_0,
            s_11_4,
            s_11_3,
            s_11_5,
            s_11_6,
            s_11_7,
            s_11_8,
            s_11_9,
        );
        // N s_11_11: return
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
        // D s_13_1: read-var esize:i64
        let s_13_1: i64 = fn_state.esize;
        // D s_13_2: cast zx s_13_1 -> i
        let s_13_2: i128 = (i128::try_from(s_13_1).unwrap());
        // D s_13_3: cast reint s_13_2 -> i64
        let s_13_3: i64 = (s_13_2 as i64);
        // D s_13_4: read-var da:i64
        let s_13_4: i64 = fn_state.da;
        // D s_13_5: read-var m:i64
        let s_13_5: i64 = fn_state.m;
        // D s_13_6: read-var n:i64
        let s_13_6: i64 = fn_state.n;
        // D s_13_7: read-var sel_a:i64
        let s_13_7: i64 = fn_state.sel_a;
        // D s_13_8: read-var sel_b:i64
        let s_13_8: i64 = fn_state.sel_b;
        // D s_13_9: read-var sub_i:u8
        let s_13_9: bool = fn_state.sub_i;
        // D s_13_10: call execute_CDOT_Z_ZZZ__(s_13_0, s_13_4, s_13_3, s_13_5, s_13_6, s_13_7, s_13_8, s_13_9)
        let s_13_10: () = execute_CDOT_Z_ZZZ__(
            state,
            tracer,
            s_13_0,
            s_13_4,
            s_13_3,
            s_13_5,
            s_13_6,
            s_13_7,
            s_13_8,
            s_13_9,
        );
        // N s_13_11: return
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
        // D s_15_1: read-var esize:i64
        let s_15_1: i64 = fn_state.esize;
        // D s_15_2: cast zx s_15_1 -> i
        let s_15_2: i128 = (i128::try_from(s_15_1).unwrap());
        // D s_15_3: cast reint s_15_2 -> i64
        let s_15_3: i64 = (s_15_2 as i64);
        // D s_15_4: read-var da:i64
        let s_15_4: i64 = fn_state.da;
        // D s_15_5: read-var m:i64
        let s_15_5: i64 = fn_state.m;
        // D s_15_6: read-var n:i64
        let s_15_6: i64 = fn_state.n;
        // D s_15_7: read-var sel_a:i64
        let s_15_7: i64 = fn_state.sel_a;
        // D s_15_8: read-var sel_b:i64
        let s_15_8: i64 = fn_state.sel_b;
        // D s_15_9: read-var sub_i:u8
        let s_15_9: bool = fn_state.sub_i;
        // D s_15_10: call execute_CDOT_Z_ZZZ__(s_15_0, s_15_4, s_15_3, s_15_5, s_15_6, s_15_7, s_15_8, s_15_9)
        let s_15_10: () = execute_CDOT_Z_ZZZ__(
            state,
            tracer,
            s_15_0,
            s_15_4,
            s_15_3,
            s_15_5,
            s_15_6,
            s_15_7,
            s_15_8,
            s_15_9,
        );
        // N s_15_11: return
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
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var gs#216650 <= s_18_0
        fn_state.gs_216650 = s_18_0;
        // N s_18_2: jump b5
        return block_5(state, tracer, fn_state);
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
        // D s_20_3: write-var gs#216649 <= s_20_2
        fn_state.gs_216649 = s_20_2;
        // N s_20_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
