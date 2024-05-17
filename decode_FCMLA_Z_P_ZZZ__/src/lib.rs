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
use execute_FCMLA_Z_P_ZZZ__::*;
use CurrentVL_read::*;
use HaveSME::*;
use common::*;
pub fn decode_FCMLA_Z_P_ZZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    size: u8,
    Zm: u8,
    rot: u8,
    Pg: u8,
    Zn: u8,
    Zda: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        esize: i64,
        neg_i: bool,
        n: i64,
        gs_181022: bool,
        VL: i64,
        sel_a: i64,
        sel_b: i64,
        neg_r: bool,
        da: i64,
        g: i64,
        size: u8,
        Zm: u8,
        rot: u8,
        Pg: u8,
        Zn: u8,
        Zda: u8,
    }
    let fn_state = FunctionState {
        size,
        Zm,
        rot,
        Pg,
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
        // S s_0_4: call HaveSVE(s_0_3)
        let s_0_4: bool = HaveSVE(state, tracer, s_0_3);
        // S s_0_5: not s_0_4
        let s_0_5: bool = !s_0_4;
        // N s_0_6: branch s_0_5 b17 b1
        if s_0_5 {
            return block_17(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#181022 <= s_1_0
        fn_state.gs_181022 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#181022:u8
        let s_2_0: bool = fn_state.gs_181022;
        // N s_2_1: branch s_2_0 b16 b3
        if s_2_0 {
            return block_16(state, tracer, fn_state);
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
        // C s_3_2: const #0u : u8
        let s_3_2: u8 = 0;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b15 b4
        if s_3_4 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var size:u8
        let s_4_0: u8 = fn_state.size;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (s_4_1.value() as i128);
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // C s_4_4: const #8s : i64
        let s_4_4: i64 = 8;
        // D s_4_5: lsl s_4_4 s_4_3
        let s_4_5: i64 = s_4_4 << s_4_3;
        // D s_4_6: write-var esize <= s_4_5
        fn_state.esize = s_4_5;
        // D s_4_7: read-var Pg:u8
        let s_4_7: u8 = fn_state.Pg;
        // D s_4_8: cast zx s_4_7 -> bv
        let s_4_8: Bits = Bits::new(s_4_7 as u128, 3u16);
        // D s_4_9: cast zx s_4_8 -> i
        let s_4_9: i128 = (s_4_8.value() as i128);
        // D s_4_10: cast reint s_4_9 -> i64
        let s_4_10: i64 = (s_4_9 as i64);
        // D s_4_11: write-var g <= s_4_10
        fn_state.g = s_4_10;
        // D s_4_12: read-var Zn:u8
        let s_4_12: u8 = fn_state.Zn;
        // D s_4_13: cast zx s_4_12 -> bv
        let s_4_13: Bits = Bits::new(s_4_12 as u128, 5u16);
        // D s_4_14: cast zx s_4_13 -> i
        let s_4_14: i128 = (s_4_13.value() as i128);
        // D s_4_15: cast reint s_4_14 -> i64
        let s_4_15: i64 = (s_4_14 as i64);
        // D s_4_16: write-var n <= s_4_15
        fn_state.n = s_4_15;
        // D s_4_17: read-var Zm:u8
        let s_4_17: u8 = fn_state.Zm;
        // D s_4_18: cast zx s_4_17 -> bv
        let s_4_18: Bits = Bits::new(s_4_17 as u128, 5u16);
        // D s_4_19: cast zx s_4_18 -> i
        let s_4_19: i128 = (s_4_18.value() as i128);
        // D s_4_20: cast reint s_4_19 -> i64
        let s_4_20: i64 = (s_4_19 as i64);
        // D s_4_21: write-var m <= s_4_20
        fn_state.m = s_4_20;
        // D s_4_22: read-var Zda:u8
        let s_4_22: u8 = fn_state.Zda;
        // D s_4_23: cast zx s_4_22 -> bv
        let s_4_23: Bits = Bits::new(s_4_22 as u128, 5u16);
        // D s_4_24: cast zx s_4_23 -> i
        let s_4_24: i128 = (s_4_23.value() as i128);
        // D s_4_25: cast reint s_4_24 -> i64
        let s_4_25: i64 = (s_4_24 as i64);
        // D s_4_26: write-var da <= s_4_25
        fn_state.da = s_4_25;
        // C s_4_27: const #0s : i
        let s_4_27: i128 = 0;
        // D s_4_28: read-var rot:u8
        let s_4_28: u8 = fn_state.rot;
        // D s_4_29: cast zx s_4_28 -> bv
        let s_4_29: Bits = Bits::new(s_4_28 as u128, 2u16);
        // C s_4_30: const #1u : u64
        let s_4_30: u64 = 1;
        // D s_4_31: bit-extract s_4_29 s_4_27 s_4_30
        let s_4_31: Bits = (Bits::new(
            ((s_4_29) >> (s_4_27)).value(),
            u16::try_from(s_4_30).unwrap(),
        ));
        // D s_4_32: cast reint s_4_31 -> u8
        let s_4_32: bool = ((s_4_31.value()) != 0);
        // C s_4_33: const #0s : i
        let s_4_33: i128 = 0;
        // C s_4_34: const #0u : u64
        let s_4_34: u64 = 0;
        // D s_4_35: cast zx s_4_32 -> u64
        let s_4_35: u64 = (s_4_32 as u64);
        // C s_4_36: const #1u : u64
        let s_4_36: u64 = 1;
        // D s_4_37: and s_4_35 s_4_36
        let s_4_37: u64 = ((s_4_35) & (s_4_36));
        // D s_4_38: cmp-eq s_4_37 s_4_36
        let s_4_38: bool = ((s_4_37) == (s_4_36));
        // D s_4_39: lsl s_4_35 s_4_33
        let s_4_39: u64 = s_4_35 << s_4_33;
        // D s_4_40: or s_4_34 s_4_39
        let s_4_40: u64 = ((s_4_34) | (s_4_39));
        // D s_4_41: cmpl s_4_39
        let s_4_41: u64 = !s_4_39;
        // D s_4_42: and s_4_34 s_4_41
        let s_4_42: u64 = ((s_4_34) & (s_4_41));
        // D s_4_43: select s_4_38 s_4_40 s_4_42
        let s_4_43: u64 = if s_4_38 { s_4_40 } else { s_4_42 };
        // D s_4_44: cast trunc s_4_43 -> u8
        let s_4_44: bool = ((s_4_43) != 0);
        // D s_4_45: cast zx s_4_44 -> bv
        let s_4_45: Bits = Bits::new(s_4_44 as u128, 1u16);
        // D s_4_46: cast zx s_4_45 -> i
        let s_4_46: i128 = (s_4_45.value() as i128);
        // D s_4_47: cast reint s_4_46 -> i64
        let s_4_47: i64 = (s_4_46 as i64);
        // D s_4_48: write-var sel_a <= s_4_47
        fn_state.sel_a = s_4_47;
        // C s_4_49: const #0s : i
        let s_4_49: i128 = 0;
        // D s_4_50: read-var rot:u8
        let s_4_50: u8 = fn_state.rot;
        // D s_4_51: cast zx s_4_50 -> bv
        let s_4_51: Bits = Bits::new(s_4_50 as u128, 2u16);
        // C s_4_52: const #1u : u64
        let s_4_52: u64 = 1;
        // D s_4_53: bit-extract s_4_51 s_4_49 s_4_52
        let s_4_53: Bits = (Bits::new(
            ((s_4_51) >> (s_4_49)).value(),
            u16::try_from(s_4_52).unwrap(),
        ));
        // D s_4_54: cast reint s_4_53 -> u8
        let s_4_54: bool = ((s_4_53.value()) != 0);
        // C s_4_55: const #0s : i
        let s_4_55: i128 = 0;
        // C s_4_56: const #0u : u64
        let s_4_56: u64 = 0;
        // D s_4_57: cast zx s_4_54 -> u64
        let s_4_57: u64 = (s_4_54 as u64);
        // C s_4_58: const #1u : u64
        let s_4_58: u64 = 1;
        // D s_4_59: and s_4_57 s_4_58
        let s_4_59: u64 = ((s_4_57) & (s_4_58));
        // D s_4_60: cmp-eq s_4_59 s_4_58
        let s_4_60: bool = ((s_4_59) == (s_4_58));
        // D s_4_61: lsl s_4_57 s_4_55
        let s_4_61: u64 = s_4_57 << s_4_55;
        // D s_4_62: or s_4_56 s_4_61
        let s_4_62: u64 = ((s_4_56) | (s_4_61));
        // D s_4_63: cmpl s_4_61
        let s_4_63: u64 = !s_4_61;
        // D s_4_64: and s_4_56 s_4_63
        let s_4_64: u64 = ((s_4_56) & (s_4_63));
        // D s_4_65: select s_4_60 s_4_62 s_4_64
        let s_4_65: u64 = if s_4_60 { s_4_62 } else { s_4_64 };
        // D s_4_66: cast trunc s_4_65 -> u8
        let s_4_66: bool = ((s_4_65) != 0);
        // D s_4_67: cast zx s_4_66 -> bv
        let s_4_67: Bits = Bits::new(s_4_66 as u128, 1u16);
        // D s_4_68: not s_4_67
        let s_4_68: Bits = !s_4_67;
        // D s_4_69: cast reint s_4_68 -> u8
        let s_4_69: bool = ((s_4_68.value()) != 0);
        // D s_4_70: cast zx s_4_69 -> bv
        let s_4_70: Bits = Bits::new(s_4_69 as u128, 1u16);
        // D s_4_71: cast zx s_4_70 -> i
        let s_4_71: i128 = (s_4_70.value() as i128);
        // D s_4_72: cast reint s_4_71 -> i64
        let s_4_72: i64 = (s_4_71 as i64);
        // D s_4_73: write-var sel_b <= s_4_72
        fn_state.sel_b = s_4_72;
        // C s_4_74: const #1s : i
        let s_4_74: i128 = 1;
        // D s_4_75: read-var rot:u8
        let s_4_75: u8 = fn_state.rot;
        // D s_4_76: cast zx s_4_75 -> bv
        let s_4_76: Bits = Bits::new(s_4_75 as u128, 2u16);
        // C s_4_77: const #1u : u64
        let s_4_77: u64 = 1;
        // D s_4_78: bit-extract s_4_76 s_4_74 s_4_77
        let s_4_78: Bits = (Bits::new(
            ((s_4_76) >> (s_4_74)).value(),
            u16::try_from(s_4_77).unwrap(),
        ));
        // D s_4_79: cast reint s_4_78 -> u8
        let s_4_79: bool = ((s_4_78.value()) != 0);
        // C s_4_80: const #0s : i
        let s_4_80: i128 = 0;
        // C s_4_81: const #0u : u64
        let s_4_81: u64 = 0;
        // D s_4_82: cast zx s_4_79 -> u64
        let s_4_82: u64 = (s_4_79 as u64);
        // C s_4_83: const #1u : u64
        let s_4_83: u64 = 1;
        // D s_4_84: and s_4_82 s_4_83
        let s_4_84: u64 = ((s_4_82) & (s_4_83));
        // D s_4_85: cmp-eq s_4_84 s_4_83
        let s_4_85: bool = ((s_4_84) == (s_4_83));
        // D s_4_86: lsl s_4_82 s_4_80
        let s_4_86: u64 = s_4_82 << s_4_80;
        // D s_4_87: or s_4_81 s_4_86
        let s_4_87: u64 = ((s_4_81) | (s_4_86));
        // D s_4_88: cmpl s_4_86
        let s_4_88: u64 = !s_4_86;
        // D s_4_89: and s_4_81 s_4_88
        let s_4_89: u64 = ((s_4_81) & (s_4_88));
        // D s_4_90: select s_4_85 s_4_87 s_4_89
        let s_4_90: u64 = if s_4_85 { s_4_87 } else { s_4_89 };
        // D s_4_91: cast trunc s_4_90 -> u8
        let s_4_91: bool = ((s_4_90) != 0);
        // D s_4_92: cast zx s_4_91 -> bv
        let s_4_92: Bits = Bits::new(s_4_91 as u128, 1u16);
        // C s_4_93: const #1u : u8
        let s_4_93: bool = true;
        // C s_4_94: cast zx s_4_93 -> bv
        let s_4_94: Bits = Bits::new(s_4_93 as u128, 1u16);
        // D s_4_95: cmp-eq s_4_92 s_4_94
        let s_4_95: bool = ((s_4_92) == (s_4_94));
        // D s_4_96: write-var neg_i <= s_4_95
        fn_state.neg_i = s_4_95;
        // C s_4_97: const #0s : i
        let s_4_97: i128 = 0;
        // D s_4_98: read-var rot:u8
        let s_4_98: u8 = fn_state.rot;
        // D s_4_99: cast zx s_4_98 -> bv
        let s_4_99: Bits = Bits::new(s_4_98 as u128, 2u16);
        // C s_4_100: const #1u : u64
        let s_4_100: u64 = 1;
        // D s_4_101: bit-extract s_4_99 s_4_97 s_4_100
        let s_4_101: Bits = (Bits::new(
            ((s_4_99) >> (s_4_97)).value(),
            u16::try_from(s_4_100).unwrap(),
        ));
        // D s_4_102: cast reint s_4_101 -> u8
        let s_4_102: bool = ((s_4_101.value()) != 0);
        // C s_4_103: const #0s : i
        let s_4_103: i128 = 0;
        // C s_4_104: const #0u : u64
        let s_4_104: u64 = 0;
        // D s_4_105: cast zx s_4_102 -> u64
        let s_4_105: u64 = (s_4_102 as u64);
        // C s_4_106: const #1u : u64
        let s_4_106: u64 = 1;
        // D s_4_107: and s_4_105 s_4_106
        let s_4_107: u64 = ((s_4_105) & (s_4_106));
        // D s_4_108: cmp-eq s_4_107 s_4_106
        let s_4_108: bool = ((s_4_107) == (s_4_106));
        // D s_4_109: lsl s_4_105 s_4_103
        let s_4_109: u64 = s_4_105 << s_4_103;
        // D s_4_110: or s_4_104 s_4_109
        let s_4_110: u64 = ((s_4_104) | (s_4_109));
        // D s_4_111: cmpl s_4_109
        let s_4_111: u64 = !s_4_109;
        // D s_4_112: and s_4_104 s_4_111
        let s_4_112: u64 = ((s_4_104) & (s_4_111));
        // D s_4_113: select s_4_108 s_4_110 s_4_112
        let s_4_113: u64 = if s_4_108 { s_4_110 } else { s_4_112 };
        // D s_4_114: cast trunc s_4_113 -> u8
        let s_4_114: bool = ((s_4_113) != 0);
        // C s_4_115: const #1s : i
        let s_4_115: i128 = 1;
        // D s_4_116: read-var rot:u8
        let s_4_116: u8 = fn_state.rot;
        // D s_4_117: cast zx s_4_116 -> bv
        let s_4_117: Bits = Bits::new(s_4_116 as u128, 2u16);
        // C s_4_118: const #1u : u64
        let s_4_118: u64 = 1;
        // D s_4_119: bit-extract s_4_117 s_4_115 s_4_118
        let s_4_119: Bits = (Bits::new(
            ((s_4_117) >> (s_4_115)).value(),
            u16::try_from(s_4_118).unwrap(),
        ));
        // D s_4_120: cast reint s_4_119 -> u8
        let s_4_120: bool = ((s_4_119.value()) != 0);
        // C s_4_121: const #0s : i
        let s_4_121: i128 = 0;
        // C s_4_122: const #0u : u64
        let s_4_122: u64 = 0;
        // D s_4_123: cast zx s_4_120 -> u64
        let s_4_123: u64 = (s_4_120 as u64);
        // C s_4_124: const #1u : u64
        let s_4_124: u64 = 1;
        // D s_4_125: and s_4_123 s_4_124
        let s_4_125: u64 = ((s_4_123) & (s_4_124));
        // D s_4_126: cmp-eq s_4_125 s_4_124
        let s_4_126: bool = ((s_4_125) == (s_4_124));
        // D s_4_127: lsl s_4_123 s_4_121
        let s_4_127: u64 = s_4_123 << s_4_121;
        // D s_4_128: or s_4_122 s_4_127
        let s_4_128: u64 = ((s_4_122) | (s_4_127));
        // D s_4_129: cmpl s_4_127
        let s_4_129: u64 = !s_4_127;
        // D s_4_130: and s_4_122 s_4_129
        let s_4_130: u64 = ((s_4_122) & (s_4_129));
        // D s_4_131: select s_4_126 s_4_128 s_4_130
        let s_4_131: u64 = if s_4_126 { s_4_128 } else { s_4_130 };
        // D s_4_132: cast trunc s_4_131 -> u8
        let s_4_132: bool = ((s_4_131) != 0);
        // D s_4_133: cast zx s_4_114 -> bv
        let s_4_133: Bits = Bits::new(s_4_114 as u128, 1u16);
        // D s_4_134: cast zx s_4_132 -> bv
        let s_4_134: Bits = Bits::new(s_4_132 as u128, 1u16);
        // D s_4_135: cmp-ne s_4_133 s_4_134
        let s_4_135: bool = ((s_4_133) != (s_4_134));
        // D s_4_136: write-var neg_r <= s_4_135
        fn_state.neg_r = s_4_135;
        // D s_4_137: read-var VL:i64
        let s_4_137: i64 = fn_state.VL;
        // C s_4_138: const #128s : i
        let s_4_138: i128 = 128;
        // D s_4_139: cast zx s_4_137 -> i
        let s_4_139: i128 = (i128::try_from(s_4_137).unwrap());
        // D s_4_140: cmp-eq s_4_139 s_4_138
        let s_4_140: bool = ((s_4_139) == (s_4_138));
        // D s_4_141: not s_4_140
        let s_4_141: bool = !s_4_140;
        // N s_4_142: branch s_4_141 b6 b5
        if s_4_141 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #128s : i64
        let s_5_0: i64 = 128;
        // D s_5_1: read-var esize:i64
        let s_5_1: i64 = fn_state.esize;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // D s_5_4: read-var da:i64
        let s_5_4: i64 = fn_state.da;
        // D s_5_5: read-var g:i64
        let s_5_5: i64 = fn_state.g;
        // D s_5_6: read-var m:i64
        let s_5_6: i64 = fn_state.m;
        // D s_5_7: read-var n:i64
        let s_5_7: i64 = fn_state.n;
        // D s_5_8: read-var neg_i:u8
        let s_5_8: bool = fn_state.neg_i;
        // D s_5_9: read-var neg_r:u8
        let s_5_9: bool = fn_state.neg_r;
        // D s_5_10: read-var sel_a:i64
        let s_5_10: i64 = fn_state.sel_a;
        // D s_5_11: read-var sel_b:i64
        let s_5_11: i64 = fn_state.sel_b;
        // D s_5_12: call execute_FCMLA_Z_P_ZZZ__(s_5_0, s_5_4, s_5_3, s_5_5, s_5_6, s_5_7, s_5_8, s_5_9, s_5_10, s_5_11)
        let s_5_12: () = execute_FCMLA_Z_P_ZZZ__(
            state,
            tracer,
            s_5_0,
            s_5_4,
            s_5_3,
            s_5_5,
            s_5_6,
            s_5_7,
            s_5_8,
            s_5_9,
            s_5_10,
            s_5_11,
        );
        // N s_5_13: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var VL:i64
        let s_6_0: i64 = fn_state.VL;
        // C s_6_1: const #256s : i
        let s_6_1: i128 = 256;
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
        // C s_7_0: const #256s : i64
        let s_7_0: i64 = 256;
        // D s_7_1: read-var esize:i64
        let s_7_1: i64 = fn_state.esize;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: cast reint s_7_2 -> i64
        let s_7_3: i64 = (s_7_2 as i64);
        // D s_7_4: read-var da:i64
        let s_7_4: i64 = fn_state.da;
        // D s_7_5: read-var g:i64
        let s_7_5: i64 = fn_state.g;
        // D s_7_6: read-var m:i64
        let s_7_6: i64 = fn_state.m;
        // D s_7_7: read-var n:i64
        let s_7_7: i64 = fn_state.n;
        // D s_7_8: read-var neg_i:u8
        let s_7_8: bool = fn_state.neg_i;
        // D s_7_9: read-var neg_r:u8
        let s_7_9: bool = fn_state.neg_r;
        // D s_7_10: read-var sel_a:i64
        let s_7_10: i64 = fn_state.sel_a;
        // D s_7_11: read-var sel_b:i64
        let s_7_11: i64 = fn_state.sel_b;
        // D s_7_12: call execute_FCMLA_Z_P_ZZZ__(s_7_0, s_7_4, s_7_3, s_7_5, s_7_6, s_7_7, s_7_8, s_7_9, s_7_10, s_7_11)
        let s_7_12: () = execute_FCMLA_Z_P_ZZZ__(
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
            s_7_10,
            s_7_11,
        );
        // N s_7_13: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var VL:i64
        let s_8_0: i64 = fn_state.VL;
        // C s_8_1: const #512s : i
        let s_8_1: i128 = 512;
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
        // C s_9_0: const #512s : i64
        let s_9_0: i64 = 512;
        // D s_9_1: read-var esize:i64
        let s_9_1: i64 = fn_state.esize;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // D s_9_4: read-var da:i64
        let s_9_4: i64 = fn_state.da;
        // D s_9_5: read-var g:i64
        let s_9_5: i64 = fn_state.g;
        // D s_9_6: read-var m:i64
        let s_9_6: i64 = fn_state.m;
        // D s_9_7: read-var n:i64
        let s_9_7: i64 = fn_state.n;
        // D s_9_8: read-var neg_i:u8
        let s_9_8: bool = fn_state.neg_i;
        // D s_9_9: read-var neg_r:u8
        let s_9_9: bool = fn_state.neg_r;
        // D s_9_10: read-var sel_a:i64
        let s_9_10: i64 = fn_state.sel_a;
        // D s_9_11: read-var sel_b:i64
        let s_9_11: i64 = fn_state.sel_b;
        // D s_9_12: call execute_FCMLA_Z_P_ZZZ__(s_9_0, s_9_4, s_9_3, s_9_5, s_9_6, s_9_7, s_9_8, s_9_9, s_9_10, s_9_11)
        let s_9_12: () = execute_FCMLA_Z_P_ZZZ__(
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
            s_9_10,
            s_9_11,
        );
        // N s_9_13: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var VL:i64
        let s_10_0: i64 = fn_state.VL;
        // C s_10_1: const #1024s : i
        let s_10_1: i128 = 1024;
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
        // C s_11_0: const #1024s : i64
        let s_11_0: i64 = 1024;
        // D s_11_1: read-var esize:i64
        let s_11_1: i64 = fn_state.esize;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: cast reint s_11_2 -> i64
        let s_11_3: i64 = (s_11_2 as i64);
        // D s_11_4: read-var da:i64
        let s_11_4: i64 = fn_state.da;
        // D s_11_5: read-var g:i64
        let s_11_5: i64 = fn_state.g;
        // D s_11_6: read-var m:i64
        let s_11_6: i64 = fn_state.m;
        // D s_11_7: read-var n:i64
        let s_11_7: i64 = fn_state.n;
        // D s_11_8: read-var neg_i:u8
        let s_11_8: bool = fn_state.neg_i;
        // D s_11_9: read-var neg_r:u8
        let s_11_9: bool = fn_state.neg_r;
        // D s_11_10: read-var sel_a:i64
        let s_11_10: i64 = fn_state.sel_a;
        // D s_11_11: read-var sel_b:i64
        let s_11_11: i64 = fn_state.sel_b;
        // D s_11_12: call execute_FCMLA_Z_P_ZZZ__(s_11_0, s_11_4, s_11_3, s_11_5, s_11_6, s_11_7, s_11_8, s_11_9, s_11_10, s_11_11)
        let s_11_12: () = execute_FCMLA_Z_P_ZZZ__(
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
            s_11_10,
            s_11_11,
        );
        // N s_11_13: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var VL:i64
        let s_12_0: i64 = fn_state.VL;
        // C s_12_1: const #2048s : i
        let s_12_1: i128 = 2048;
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
        // C s_13_0: const #2048s : i64
        let s_13_0: i64 = 2048;
        // D s_13_1: read-var esize:i64
        let s_13_1: i64 = fn_state.esize;
        // D s_13_2: cast zx s_13_1 -> i
        let s_13_2: i128 = (i128::try_from(s_13_1).unwrap());
        // D s_13_3: cast reint s_13_2 -> i64
        let s_13_3: i64 = (s_13_2 as i64);
        // D s_13_4: read-var da:i64
        let s_13_4: i64 = fn_state.da;
        // D s_13_5: read-var g:i64
        let s_13_5: i64 = fn_state.g;
        // D s_13_6: read-var m:i64
        let s_13_6: i64 = fn_state.m;
        // D s_13_7: read-var n:i64
        let s_13_7: i64 = fn_state.n;
        // D s_13_8: read-var neg_i:u8
        let s_13_8: bool = fn_state.neg_i;
        // D s_13_9: read-var neg_r:u8
        let s_13_9: bool = fn_state.neg_r;
        // D s_13_10: read-var sel_a:i64
        let s_13_10: i64 = fn_state.sel_a;
        // D s_13_11: read-var sel_b:i64
        let s_13_11: i64 = fn_state.sel_b;
        // D s_13_12: call execute_FCMLA_Z_P_ZZZ__(s_13_0, s_13_4, s_13_3, s_13_5, s_13_6, s_13_7, s_13_8, s_13_9, s_13_10, s_13_11)
        let s_13_12: () = execute_FCMLA_Z_P_ZZZ__(
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
            s_13_10,
            s_13_11,
        );
        // N s_13_13: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: return
        return;
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
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: panic
        panic!("{:?}", ());
        // N s_16_1: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call HaveSME(s_17_0)
        let s_17_1: bool = HaveSME(state, tracer, s_17_0);
        // S s_17_2: not s_17_1
        let s_17_2: bool = !s_17_1;
        // D s_17_3: write-var gs#181022 <= s_17_2
        fn_state.gs_181022 = s_17_2;
        // N s_17_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
