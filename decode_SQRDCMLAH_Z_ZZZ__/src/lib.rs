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
use CurrentVL_read::*;
use HaveSVE2::*;
use execute_SQRDCMLAH_Z_ZZZ__::*;
use HaveSME::*;
use common::*;
pub fn decode_SQRDCMLAH_Z_ZZZ__<T: Tracer>(
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
        n: i64,
        sub_i: bool,
        gs_210527: bool,
        VL: i64,
        sel_a: i64,
        sel_b: i64,
        sub_r: bool,
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
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#210527 <= s_1_0
        fn_state.gs_210527 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#210527:u8
        let s_2_0: bool = fn_state.gs_210527;
        // N s_2_1: branch s_2_0 b14 b3
        if s_2_0 {
            return block_14(state, tracer, fn_state);
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
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // C s_3_4: const #8s : i64
        let s_3_4: i64 = 8;
        // D s_3_5: lsl s_3_4 s_3_3
        let s_3_5: i64 = s_3_4 << s_3_3;
        // D s_3_6: write-var esize <= s_3_5
        fn_state.esize = s_3_5;
        // D s_3_7: read-var Zn:u8
        let s_3_7: u8 = fn_state.Zn;
        // D s_3_8: cast zx s_3_7 -> bv
        let s_3_8: Bits = Bits::new(s_3_7 as u128, 5u16);
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (s_3_8.value() as i128);
        // D s_3_10: cast reint s_3_9 -> i64
        let s_3_10: i64 = (s_3_9 as i64);
        // D s_3_11: write-var n <= s_3_10
        fn_state.n = s_3_10;
        // D s_3_12: read-var Zm:u8
        let s_3_12: u8 = fn_state.Zm;
        // D s_3_13: cast zx s_3_12 -> bv
        let s_3_13: Bits = Bits::new(s_3_12 as u128, 5u16);
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (s_3_13.value() as i128);
        // D s_3_15: cast reint s_3_14 -> i64
        let s_3_15: i64 = (s_3_14 as i64);
        // D s_3_16: write-var m <= s_3_15
        fn_state.m = s_3_15;
        // D s_3_17: read-var Zda:u8
        let s_3_17: u8 = fn_state.Zda;
        // D s_3_18: cast zx s_3_17 -> bv
        let s_3_18: Bits = Bits::new(s_3_17 as u128, 5u16);
        // D s_3_19: cast zx s_3_18 -> i
        let s_3_19: i128 = (s_3_18.value() as i128);
        // D s_3_20: cast reint s_3_19 -> i64
        let s_3_20: i64 = (s_3_19 as i64);
        // D s_3_21: write-var da <= s_3_20
        fn_state.da = s_3_20;
        // C s_3_22: const #0s : i
        let s_3_22: i128 = 0;
        // D s_3_23: read-var rot:u8
        let s_3_23: u8 = fn_state.rot;
        // D s_3_24: cast zx s_3_23 -> bv
        let s_3_24: Bits = Bits::new(s_3_23 as u128, 2u16);
        // C s_3_25: const #1u : u64
        let s_3_25: u64 = 1;
        // D s_3_26: bit-extract s_3_24 s_3_22 s_3_25
        let s_3_26: Bits = (Bits::new(
            ((s_3_24) >> (s_3_22)).value(),
            u16::try_from(s_3_25).unwrap(),
        ));
        // D s_3_27: cast reint s_3_26 -> u8
        let s_3_27: bool = ((s_3_26.value()) != 0);
        // C s_3_28: const #0s : i
        let s_3_28: i128 = 0;
        // C s_3_29: const #0u : u64
        let s_3_29: u64 = 0;
        // D s_3_30: cast zx s_3_27 -> u64
        let s_3_30: u64 = (s_3_27 as u64);
        // C s_3_31: const #1u : u64
        let s_3_31: u64 = 1;
        // D s_3_32: and s_3_30 s_3_31
        let s_3_32: u64 = ((s_3_30) & (s_3_31));
        // D s_3_33: cmp-eq s_3_32 s_3_31
        let s_3_33: bool = ((s_3_32) == (s_3_31));
        // D s_3_34: lsl s_3_30 s_3_28
        let s_3_34: u64 = s_3_30 << s_3_28;
        // D s_3_35: or s_3_29 s_3_34
        let s_3_35: u64 = ((s_3_29) | (s_3_34));
        // D s_3_36: cmpl s_3_34
        let s_3_36: u64 = !s_3_34;
        // D s_3_37: and s_3_29 s_3_36
        let s_3_37: u64 = ((s_3_29) & (s_3_36));
        // D s_3_38: select s_3_33 s_3_35 s_3_37
        let s_3_38: u64 = if s_3_33 { s_3_35 } else { s_3_37 };
        // D s_3_39: cast trunc s_3_38 -> u8
        let s_3_39: bool = ((s_3_38) != 0);
        // D s_3_40: cast zx s_3_39 -> bv
        let s_3_40: Bits = Bits::new(s_3_39 as u128, 1u16);
        // D s_3_41: cast zx s_3_40 -> i
        let s_3_41: i128 = (s_3_40.value() as i128);
        // D s_3_42: cast reint s_3_41 -> i64
        let s_3_42: i64 = (s_3_41 as i64);
        // D s_3_43: write-var sel_a <= s_3_42
        fn_state.sel_a = s_3_42;
        // C s_3_44: const #0s : i
        let s_3_44: i128 = 0;
        // D s_3_45: read-var rot:u8
        let s_3_45: u8 = fn_state.rot;
        // D s_3_46: cast zx s_3_45 -> bv
        let s_3_46: Bits = Bits::new(s_3_45 as u128, 2u16);
        // C s_3_47: const #1u : u64
        let s_3_47: u64 = 1;
        // D s_3_48: bit-extract s_3_46 s_3_44 s_3_47
        let s_3_48: Bits = (Bits::new(
            ((s_3_46) >> (s_3_44)).value(),
            u16::try_from(s_3_47).unwrap(),
        ));
        // D s_3_49: cast reint s_3_48 -> u8
        let s_3_49: bool = ((s_3_48.value()) != 0);
        // C s_3_50: const #0s : i
        let s_3_50: i128 = 0;
        // C s_3_51: const #0u : u64
        let s_3_51: u64 = 0;
        // D s_3_52: cast zx s_3_49 -> u64
        let s_3_52: u64 = (s_3_49 as u64);
        // C s_3_53: const #1u : u64
        let s_3_53: u64 = 1;
        // D s_3_54: and s_3_52 s_3_53
        let s_3_54: u64 = ((s_3_52) & (s_3_53));
        // D s_3_55: cmp-eq s_3_54 s_3_53
        let s_3_55: bool = ((s_3_54) == (s_3_53));
        // D s_3_56: lsl s_3_52 s_3_50
        let s_3_56: u64 = s_3_52 << s_3_50;
        // D s_3_57: or s_3_51 s_3_56
        let s_3_57: u64 = ((s_3_51) | (s_3_56));
        // D s_3_58: cmpl s_3_56
        let s_3_58: u64 = !s_3_56;
        // D s_3_59: and s_3_51 s_3_58
        let s_3_59: u64 = ((s_3_51) & (s_3_58));
        // D s_3_60: select s_3_55 s_3_57 s_3_59
        let s_3_60: u64 = if s_3_55 { s_3_57 } else { s_3_59 };
        // D s_3_61: cast trunc s_3_60 -> u8
        let s_3_61: bool = ((s_3_60) != 0);
        // D s_3_62: cast zx s_3_61 -> bv
        let s_3_62: Bits = Bits::new(s_3_61 as u128, 1u16);
        // D s_3_63: not s_3_62
        let s_3_63: Bits = !s_3_62;
        // D s_3_64: cast reint s_3_63 -> u8
        let s_3_64: bool = ((s_3_63.value()) != 0);
        // D s_3_65: cast zx s_3_64 -> bv
        let s_3_65: Bits = Bits::new(s_3_64 as u128, 1u16);
        // D s_3_66: cast zx s_3_65 -> i
        let s_3_66: i128 = (s_3_65.value() as i128);
        // D s_3_67: cast reint s_3_66 -> i64
        let s_3_67: i64 = (s_3_66 as i64);
        // D s_3_68: write-var sel_b <= s_3_67
        fn_state.sel_b = s_3_67;
        // C s_3_69: const #0s : i
        let s_3_69: i128 = 0;
        // D s_3_70: read-var rot:u8
        let s_3_70: u8 = fn_state.rot;
        // D s_3_71: cast zx s_3_70 -> bv
        let s_3_71: Bits = Bits::new(s_3_70 as u128, 2u16);
        // C s_3_72: const #1u : u64
        let s_3_72: u64 = 1;
        // D s_3_73: bit-extract s_3_71 s_3_69 s_3_72
        let s_3_73: Bits = (Bits::new(
            ((s_3_71) >> (s_3_69)).value(),
            u16::try_from(s_3_72).unwrap(),
        ));
        // D s_3_74: cast reint s_3_73 -> u8
        let s_3_74: bool = ((s_3_73.value()) != 0);
        // C s_3_75: const #0s : i
        let s_3_75: i128 = 0;
        // C s_3_76: const #0u : u64
        let s_3_76: u64 = 0;
        // D s_3_77: cast zx s_3_74 -> u64
        let s_3_77: u64 = (s_3_74 as u64);
        // C s_3_78: const #1u : u64
        let s_3_78: u64 = 1;
        // D s_3_79: and s_3_77 s_3_78
        let s_3_79: u64 = ((s_3_77) & (s_3_78));
        // D s_3_80: cmp-eq s_3_79 s_3_78
        let s_3_80: bool = ((s_3_79) == (s_3_78));
        // D s_3_81: lsl s_3_77 s_3_75
        let s_3_81: u64 = s_3_77 << s_3_75;
        // D s_3_82: or s_3_76 s_3_81
        let s_3_82: u64 = ((s_3_76) | (s_3_81));
        // D s_3_83: cmpl s_3_81
        let s_3_83: u64 = !s_3_81;
        // D s_3_84: and s_3_76 s_3_83
        let s_3_84: u64 = ((s_3_76) & (s_3_83));
        // D s_3_85: select s_3_80 s_3_82 s_3_84
        let s_3_85: u64 = if s_3_80 { s_3_82 } else { s_3_84 };
        // D s_3_86: cast trunc s_3_85 -> u8
        let s_3_86: bool = ((s_3_85) != 0);
        // C s_3_87: const #1s : i
        let s_3_87: i128 = 1;
        // D s_3_88: read-var rot:u8
        let s_3_88: u8 = fn_state.rot;
        // D s_3_89: cast zx s_3_88 -> bv
        let s_3_89: Bits = Bits::new(s_3_88 as u128, 2u16);
        // C s_3_90: const #1u : u64
        let s_3_90: u64 = 1;
        // D s_3_91: bit-extract s_3_89 s_3_87 s_3_90
        let s_3_91: Bits = (Bits::new(
            ((s_3_89) >> (s_3_87)).value(),
            u16::try_from(s_3_90).unwrap(),
        ));
        // D s_3_92: cast reint s_3_91 -> u8
        let s_3_92: bool = ((s_3_91.value()) != 0);
        // C s_3_93: const #0s : i
        let s_3_93: i128 = 0;
        // C s_3_94: const #0u : u64
        let s_3_94: u64 = 0;
        // D s_3_95: cast zx s_3_92 -> u64
        let s_3_95: u64 = (s_3_92 as u64);
        // C s_3_96: const #1u : u64
        let s_3_96: u64 = 1;
        // D s_3_97: and s_3_95 s_3_96
        let s_3_97: u64 = ((s_3_95) & (s_3_96));
        // D s_3_98: cmp-eq s_3_97 s_3_96
        let s_3_98: bool = ((s_3_97) == (s_3_96));
        // D s_3_99: lsl s_3_95 s_3_93
        let s_3_99: u64 = s_3_95 << s_3_93;
        // D s_3_100: or s_3_94 s_3_99
        let s_3_100: u64 = ((s_3_94) | (s_3_99));
        // D s_3_101: cmpl s_3_99
        let s_3_101: u64 = !s_3_99;
        // D s_3_102: and s_3_94 s_3_101
        let s_3_102: u64 = ((s_3_94) & (s_3_101));
        // D s_3_103: select s_3_98 s_3_100 s_3_102
        let s_3_103: u64 = if s_3_98 { s_3_100 } else { s_3_102 };
        // D s_3_104: cast trunc s_3_103 -> u8
        let s_3_104: bool = ((s_3_103) != 0);
        // D s_3_105: cast zx s_3_86 -> bv
        let s_3_105: Bits = Bits::new(s_3_86 as u128, 1u16);
        // D s_3_106: cast zx s_3_104 -> bv
        let s_3_106: Bits = Bits::new(s_3_104 as u128, 1u16);
        // D s_3_107: cmp-ne s_3_105 s_3_106
        let s_3_107: bool = ((s_3_105) != (s_3_106));
        // D s_3_108: write-var sub_r <= s_3_107
        fn_state.sub_r = s_3_107;
        // C s_3_109: const #1s : i
        let s_3_109: i128 = 1;
        // D s_3_110: read-var rot:u8
        let s_3_110: u8 = fn_state.rot;
        // D s_3_111: cast zx s_3_110 -> bv
        let s_3_111: Bits = Bits::new(s_3_110 as u128, 2u16);
        // C s_3_112: const #1u : u64
        let s_3_112: u64 = 1;
        // D s_3_113: bit-extract s_3_111 s_3_109 s_3_112
        let s_3_113: Bits = (Bits::new(
            ((s_3_111) >> (s_3_109)).value(),
            u16::try_from(s_3_112).unwrap(),
        ));
        // D s_3_114: cast reint s_3_113 -> u8
        let s_3_114: bool = ((s_3_113.value()) != 0);
        // C s_3_115: const #0s : i
        let s_3_115: i128 = 0;
        // C s_3_116: const #0u : u64
        let s_3_116: u64 = 0;
        // D s_3_117: cast zx s_3_114 -> u64
        let s_3_117: u64 = (s_3_114 as u64);
        // C s_3_118: const #1u : u64
        let s_3_118: u64 = 1;
        // D s_3_119: and s_3_117 s_3_118
        let s_3_119: u64 = ((s_3_117) & (s_3_118));
        // D s_3_120: cmp-eq s_3_119 s_3_118
        let s_3_120: bool = ((s_3_119) == (s_3_118));
        // D s_3_121: lsl s_3_117 s_3_115
        let s_3_121: u64 = s_3_117 << s_3_115;
        // D s_3_122: or s_3_116 s_3_121
        let s_3_122: u64 = ((s_3_116) | (s_3_121));
        // D s_3_123: cmpl s_3_121
        let s_3_123: u64 = !s_3_121;
        // D s_3_124: and s_3_116 s_3_123
        let s_3_124: u64 = ((s_3_116) & (s_3_123));
        // D s_3_125: select s_3_120 s_3_122 s_3_124
        let s_3_125: u64 = if s_3_120 { s_3_122 } else { s_3_124 };
        // D s_3_126: cast trunc s_3_125 -> u8
        let s_3_126: bool = ((s_3_125) != 0);
        // D s_3_127: cast zx s_3_126 -> bv
        let s_3_127: Bits = Bits::new(s_3_126 as u128, 1u16);
        // C s_3_128: const #1u : u8
        let s_3_128: bool = true;
        // C s_3_129: cast zx s_3_128 -> bv
        let s_3_129: Bits = Bits::new(s_3_128 as u128, 1u16);
        // D s_3_130: cmp-eq s_3_127 s_3_129
        let s_3_130: bool = ((s_3_127) == (s_3_129));
        // D s_3_131: write-var sub_i <= s_3_130
        fn_state.sub_i = s_3_130;
        // D s_3_132: read-var VL:i64
        let s_3_132: i64 = fn_state.VL;
        // C s_3_133: const #128s : i
        let s_3_133: i128 = 128;
        // D s_3_134: cast zx s_3_132 -> i
        let s_3_134: i128 = (i128::try_from(s_3_132).unwrap());
        // D s_3_135: cmp-eq s_3_134 s_3_133
        let s_3_135: bool = ((s_3_134) == (s_3_133));
        // D s_3_136: not s_3_135
        let s_3_136: bool = !s_3_135;
        // N s_3_137: branch s_3_136 b5 b4
        if s_3_136 {
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
        // D s_4_1: read-var esize:i64
        let s_4_1: i64 = fn_state.esize;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // D s_4_4: read-var da:i64
        let s_4_4: i64 = fn_state.da;
        // D s_4_5: read-var m:i64
        let s_4_5: i64 = fn_state.m;
        // D s_4_6: read-var n:i64
        let s_4_6: i64 = fn_state.n;
        // D s_4_7: read-var sel_a:i64
        let s_4_7: i64 = fn_state.sel_a;
        // D s_4_8: read-var sel_b:i64
        let s_4_8: i64 = fn_state.sel_b;
        // D s_4_9: read-var sub_i:u8
        let s_4_9: bool = fn_state.sub_i;
        // D s_4_10: read-var sub_r:u8
        let s_4_10: bool = fn_state.sub_r;
        // D s_4_11: call execute_SQRDCMLAH_Z_ZZZ__(s_4_0, s_4_4, s_4_3, s_4_5, s_4_6, s_4_7, s_4_8, s_4_9, s_4_10)
        let s_4_11: () = execute_SQRDCMLAH_Z_ZZZ__(
            state,
            tracer,
            s_4_0,
            s_4_4,
            s_4_3,
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
        // D s_6_1: read-var esize:i64
        let s_6_1: i64 = fn_state.esize;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // D s_6_4: read-var da:i64
        let s_6_4: i64 = fn_state.da;
        // D s_6_5: read-var m:i64
        let s_6_5: i64 = fn_state.m;
        // D s_6_6: read-var n:i64
        let s_6_6: i64 = fn_state.n;
        // D s_6_7: read-var sel_a:i64
        let s_6_7: i64 = fn_state.sel_a;
        // D s_6_8: read-var sel_b:i64
        let s_6_8: i64 = fn_state.sel_b;
        // D s_6_9: read-var sub_i:u8
        let s_6_9: bool = fn_state.sub_i;
        // D s_6_10: read-var sub_r:u8
        let s_6_10: bool = fn_state.sub_r;
        // D s_6_11: call execute_SQRDCMLAH_Z_ZZZ__(s_6_0, s_6_4, s_6_3, s_6_5, s_6_6, s_6_7, s_6_8, s_6_9, s_6_10)
        let s_6_11: () = execute_SQRDCMLAH_Z_ZZZ__(
            state,
            tracer,
            s_6_0,
            s_6_4,
            s_6_3,
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
        // D s_8_1: read-var esize:i64
        let s_8_1: i64 = fn_state.esize;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: cast reint s_8_2 -> i64
        let s_8_3: i64 = (s_8_2 as i64);
        // D s_8_4: read-var da:i64
        let s_8_4: i64 = fn_state.da;
        // D s_8_5: read-var m:i64
        let s_8_5: i64 = fn_state.m;
        // D s_8_6: read-var n:i64
        let s_8_6: i64 = fn_state.n;
        // D s_8_7: read-var sel_a:i64
        let s_8_7: i64 = fn_state.sel_a;
        // D s_8_8: read-var sel_b:i64
        let s_8_8: i64 = fn_state.sel_b;
        // D s_8_9: read-var sub_i:u8
        let s_8_9: bool = fn_state.sub_i;
        // D s_8_10: read-var sub_r:u8
        let s_8_10: bool = fn_state.sub_r;
        // D s_8_11: call execute_SQRDCMLAH_Z_ZZZ__(s_8_0, s_8_4, s_8_3, s_8_5, s_8_6, s_8_7, s_8_8, s_8_9, s_8_10)
        let s_8_11: () = execute_SQRDCMLAH_Z_ZZZ__(
            state,
            tracer,
            s_8_0,
            s_8_4,
            s_8_3,
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
        // D s_10_1: read-var esize:i64
        let s_10_1: i64 = fn_state.esize;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: cast reint s_10_2 -> i64
        let s_10_3: i64 = (s_10_2 as i64);
        // D s_10_4: read-var da:i64
        let s_10_4: i64 = fn_state.da;
        // D s_10_5: read-var m:i64
        let s_10_5: i64 = fn_state.m;
        // D s_10_6: read-var n:i64
        let s_10_6: i64 = fn_state.n;
        // D s_10_7: read-var sel_a:i64
        let s_10_7: i64 = fn_state.sel_a;
        // D s_10_8: read-var sel_b:i64
        let s_10_8: i64 = fn_state.sel_b;
        // D s_10_9: read-var sub_i:u8
        let s_10_9: bool = fn_state.sub_i;
        // D s_10_10: read-var sub_r:u8
        let s_10_10: bool = fn_state.sub_r;
        // D s_10_11: call execute_SQRDCMLAH_Z_ZZZ__(s_10_0, s_10_4, s_10_3, s_10_5, s_10_6, s_10_7, s_10_8, s_10_9, s_10_10)
        let s_10_11: () = execute_SQRDCMLAH_Z_ZZZ__(
            state,
            tracer,
            s_10_0,
            s_10_4,
            s_10_3,
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
        // D s_12_1: read-var esize:i64
        let s_12_1: i64 = fn_state.esize;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: cast reint s_12_2 -> i64
        let s_12_3: i64 = (s_12_2 as i64);
        // D s_12_4: read-var da:i64
        let s_12_4: i64 = fn_state.da;
        // D s_12_5: read-var m:i64
        let s_12_5: i64 = fn_state.m;
        // D s_12_6: read-var n:i64
        let s_12_6: i64 = fn_state.n;
        // D s_12_7: read-var sel_a:i64
        let s_12_7: i64 = fn_state.sel_a;
        // D s_12_8: read-var sel_b:i64
        let s_12_8: i64 = fn_state.sel_b;
        // D s_12_9: read-var sub_i:u8
        let s_12_9: bool = fn_state.sub_i;
        // D s_12_10: read-var sub_r:u8
        let s_12_10: bool = fn_state.sub_r;
        // D s_12_11: call execute_SQRDCMLAH_Z_ZZZ__(s_12_0, s_12_4, s_12_3, s_12_5, s_12_6, s_12_7, s_12_8, s_12_9, s_12_10)
        let s_12_11: () = execute_SQRDCMLAH_Z_ZZZ__(
            state,
            tracer,
            s_12_0,
            s_12_4,
            s_12_3,
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
        // N s_14_0: panic
        panic!("{:?}", ());
        // N s_14_1: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call HaveSME(s_15_0)
        let s_15_1: bool = HaveSME(state, tracer, s_15_0);
        // S s_15_2: not s_15_1
        let s_15_2: bool = !s_15_1;
        // D s_15_3: write-var gs#210527 <= s_15_2
        fn_state.gs_210527 = s_15_2;
        // N s_15_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
