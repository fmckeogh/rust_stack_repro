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
use execute_SQRDCMLAH_Z_ZZZi_H::*;
use HaveSVE2::*;
use HaveSME::*;
use common::*;
pub fn decode_SQRDCMLAH_Z_ZZZi_H<T: Tracer>(
    state: &mut State,
    tracer: &T,
    size: u8,
    i2: u8,
    Zm: u8,
    rot: u8,
    Zn: u8,
    Zda: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        n: i64,
        index: i64,
        sub_i: bool,
        VL: i64,
        sel_a: i64,
        sel_b: i64,
        gs_210630: bool,
        sub_r: bool,
        da: i64,
        size: u8,
        i2: u8,
        Zm: u8,
        rot: u8,
        Zn: u8,
        Zda: u8,
    }
    let fn_state = FunctionState {
        size,
        i2,
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
        // D s_1_1: write-var gs#210630 <= s_1_0
        fn_state.gs_210630 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#210630:u8
        let s_2_0: bool = fn_state.gs_210630;
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
        // D s_3_0: read-var i2:u8
        let s_3_0: u8 = fn_state.i2;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // D s_3_4: write-var index <= s_3_3
        fn_state.index = s_3_3;
        // D s_3_5: read-var Zn:u8
        let s_3_5: u8 = fn_state.Zn;
        // D s_3_6: cast zx s_3_5 -> bv
        let s_3_6: Bits = Bits::new(s_3_5 as u128, 5u16);
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (s_3_6.value() as i128);
        // D s_3_8: cast reint s_3_7 -> i64
        let s_3_8: i64 = (s_3_7 as i64);
        // D s_3_9: write-var n <= s_3_8
        fn_state.n = s_3_8;
        // D s_3_10: read-var Zm:u8
        let s_3_10: u8 = fn_state.Zm;
        // D s_3_11: cast zx s_3_10 -> bv
        let s_3_11: Bits = Bits::new(s_3_10 as u128, 3u16);
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (s_3_11.value() as i128);
        // D s_3_13: cast reint s_3_12 -> i64
        let s_3_13: i64 = (s_3_12 as i64);
        // D s_3_14: write-var m <= s_3_13
        fn_state.m = s_3_13;
        // D s_3_15: read-var Zda:u8
        let s_3_15: u8 = fn_state.Zda;
        // D s_3_16: cast zx s_3_15 -> bv
        let s_3_16: Bits = Bits::new(s_3_15 as u128, 5u16);
        // D s_3_17: cast zx s_3_16 -> i
        let s_3_17: i128 = (s_3_16.value() as i128);
        // D s_3_18: cast reint s_3_17 -> i64
        let s_3_18: i64 = (s_3_17 as i64);
        // D s_3_19: write-var da <= s_3_18
        fn_state.da = s_3_18;
        // C s_3_20: const #0s : i
        let s_3_20: i128 = 0;
        // D s_3_21: read-var rot:u8
        let s_3_21: u8 = fn_state.rot;
        // D s_3_22: cast zx s_3_21 -> bv
        let s_3_22: Bits = Bits::new(s_3_21 as u128, 2u16);
        // C s_3_23: const #1u : u64
        let s_3_23: u64 = 1;
        // D s_3_24: bit-extract s_3_22 s_3_20 s_3_23
        let s_3_24: Bits = (Bits::new(
            ((s_3_22) >> (s_3_20)).value(),
            u16::try_from(s_3_23).unwrap(),
        ));
        // D s_3_25: cast reint s_3_24 -> u8
        let s_3_25: bool = ((s_3_24.value()) != 0);
        // C s_3_26: const #0s : i
        let s_3_26: i128 = 0;
        // C s_3_27: const #0u : u64
        let s_3_27: u64 = 0;
        // D s_3_28: cast zx s_3_25 -> u64
        let s_3_28: u64 = (s_3_25 as u64);
        // C s_3_29: const #1u : u64
        let s_3_29: u64 = 1;
        // D s_3_30: and s_3_28 s_3_29
        let s_3_30: u64 = ((s_3_28) & (s_3_29));
        // D s_3_31: cmp-eq s_3_30 s_3_29
        let s_3_31: bool = ((s_3_30) == (s_3_29));
        // D s_3_32: lsl s_3_28 s_3_26
        let s_3_32: u64 = s_3_28 << s_3_26;
        // D s_3_33: or s_3_27 s_3_32
        let s_3_33: u64 = ((s_3_27) | (s_3_32));
        // D s_3_34: cmpl s_3_32
        let s_3_34: u64 = !s_3_32;
        // D s_3_35: and s_3_27 s_3_34
        let s_3_35: u64 = ((s_3_27) & (s_3_34));
        // D s_3_36: select s_3_31 s_3_33 s_3_35
        let s_3_36: u64 = if s_3_31 { s_3_33 } else { s_3_35 };
        // D s_3_37: cast trunc s_3_36 -> u8
        let s_3_37: bool = ((s_3_36) != 0);
        // D s_3_38: cast zx s_3_37 -> bv
        let s_3_38: Bits = Bits::new(s_3_37 as u128, 1u16);
        // D s_3_39: cast zx s_3_38 -> i
        let s_3_39: i128 = (s_3_38.value() as i128);
        // D s_3_40: cast reint s_3_39 -> i64
        let s_3_40: i64 = (s_3_39 as i64);
        // D s_3_41: write-var sel_a <= s_3_40
        fn_state.sel_a = s_3_40;
        // C s_3_42: const #0s : i
        let s_3_42: i128 = 0;
        // D s_3_43: read-var rot:u8
        let s_3_43: u8 = fn_state.rot;
        // D s_3_44: cast zx s_3_43 -> bv
        let s_3_44: Bits = Bits::new(s_3_43 as u128, 2u16);
        // C s_3_45: const #1u : u64
        let s_3_45: u64 = 1;
        // D s_3_46: bit-extract s_3_44 s_3_42 s_3_45
        let s_3_46: Bits = (Bits::new(
            ((s_3_44) >> (s_3_42)).value(),
            u16::try_from(s_3_45).unwrap(),
        ));
        // D s_3_47: cast reint s_3_46 -> u8
        let s_3_47: bool = ((s_3_46.value()) != 0);
        // C s_3_48: const #0s : i
        let s_3_48: i128 = 0;
        // C s_3_49: const #0u : u64
        let s_3_49: u64 = 0;
        // D s_3_50: cast zx s_3_47 -> u64
        let s_3_50: u64 = (s_3_47 as u64);
        // C s_3_51: const #1u : u64
        let s_3_51: u64 = 1;
        // D s_3_52: and s_3_50 s_3_51
        let s_3_52: u64 = ((s_3_50) & (s_3_51));
        // D s_3_53: cmp-eq s_3_52 s_3_51
        let s_3_53: bool = ((s_3_52) == (s_3_51));
        // D s_3_54: lsl s_3_50 s_3_48
        let s_3_54: u64 = s_3_50 << s_3_48;
        // D s_3_55: or s_3_49 s_3_54
        let s_3_55: u64 = ((s_3_49) | (s_3_54));
        // D s_3_56: cmpl s_3_54
        let s_3_56: u64 = !s_3_54;
        // D s_3_57: and s_3_49 s_3_56
        let s_3_57: u64 = ((s_3_49) & (s_3_56));
        // D s_3_58: select s_3_53 s_3_55 s_3_57
        let s_3_58: u64 = if s_3_53 { s_3_55 } else { s_3_57 };
        // D s_3_59: cast trunc s_3_58 -> u8
        let s_3_59: bool = ((s_3_58) != 0);
        // D s_3_60: cast zx s_3_59 -> bv
        let s_3_60: Bits = Bits::new(s_3_59 as u128, 1u16);
        // D s_3_61: not s_3_60
        let s_3_61: Bits = !s_3_60;
        // D s_3_62: cast reint s_3_61 -> u8
        let s_3_62: bool = ((s_3_61.value()) != 0);
        // D s_3_63: cast zx s_3_62 -> bv
        let s_3_63: Bits = Bits::new(s_3_62 as u128, 1u16);
        // D s_3_64: cast zx s_3_63 -> i
        let s_3_64: i128 = (s_3_63.value() as i128);
        // D s_3_65: cast reint s_3_64 -> i64
        let s_3_65: i64 = (s_3_64 as i64);
        // D s_3_66: write-var sel_b <= s_3_65
        fn_state.sel_b = s_3_65;
        // C s_3_67: const #0s : i
        let s_3_67: i128 = 0;
        // D s_3_68: read-var rot:u8
        let s_3_68: u8 = fn_state.rot;
        // D s_3_69: cast zx s_3_68 -> bv
        let s_3_69: Bits = Bits::new(s_3_68 as u128, 2u16);
        // C s_3_70: const #1u : u64
        let s_3_70: u64 = 1;
        // D s_3_71: bit-extract s_3_69 s_3_67 s_3_70
        let s_3_71: Bits = (Bits::new(
            ((s_3_69) >> (s_3_67)).value(),
            u16::try_from(s_3_70).unwrap(),
        ));
        // D s_3_72: cast reint s_3_71 -> u8
        let s_3_72: bool = ((s_3_71.value()) != 0);
        // C s_3_73: const #0s : i
        let s_3_73: i128 = 0;
        // C s_3_74: const #0u : u64
        let s_3_74: u64 = 0;
        // D s_3_75: cast zx s_3_72 -> u64
        let s_3_75: u64 = (s_3_72 as u64);
        // C s_3_76: const #1u : u64
        let s_3_76: u64 = 1;
        // D s_3_77: and s_3_75 s_3_76
        let s_3_77: u64 = ((s_3_75) & (s_3_76));
        // D s_3_78: cmp-eq s_3_77 s_3_76
        let s_3_78: bool = ((s_3_77) == (s_3_76));
        // D s_3_79: lsl s_3_75 s_3_73
        let s_3_79: u64 = s_3_75 << s_3_73;
        // D s_3_80: or s_3_74 s_3_79
        let s_3_80: u64 = ((s_3_74) | (s_3_79));
        // D s_3_81: cmpl s_3_79
        let s_3_81: u64 = !s_3_79;
        // D s_3_82: and s_3_74 s_3_81
        let s_3_82: u64 = ((s_3_74) & (s_3_81));
        // D s_3_83: select s_3_78 s_3_80 s_3_82
        let s_3_83: u64 = if s_3_78 { s_3_80 } else { s_3_82 };
        // D s_3_84: cast trunc s_3_83 -> u8
        let s_3_84: bool = ((s_3_83) != 0);
        // C s_3_85: const #1s : i
        let s_3_85: i128 = 1;
        // D s_3_86: read-var rot:u8
        let s_3_86: u8 = fn_state.rot;
        // D s_3_87: cast zx s_3_86 -> bv
        let s_3_87: Bits = Bits::new(s_3_86 as u128, 2u16);
        // C s_3_88: const #1u : u64
        let s_3_88: u64 = 1;
        // D s_3_89: bit-extract s_3_87 s_3_85 s_3_88
        let s_3_89: Bits = (Bits::new(
            ((s_3_87) >> (s_3_85)).value(),
            u16::try_from(s_3_88).unwrap(),
        ));
        // D s_3_90: cast reint s_3_89 -> u8
        let s_3_90: bool = ((s_3_89.value()) != 0);
        // C s_3_91: const #0s : i
        let s_3_91: i128 = 0;
        // C s_3_92: const #0u : u64
        let s_3_92: u64 = 0;
        // D s_3_93: cast zx s_3_90 -> u64
        let s_3_93: u64 = (s_3_90 as u64);
        // C s_3_94: const #1u : u64
        let s_3_94: u64 = 1;
        // D s_3_95: and s_3_93 s_3_94
        let s_3_95: u64 = ((s_3_93) & (s_3_94));
        // D s_3_96: cmp-eq s_3_95 s_3_94
        let s_3_96: bool = ((s_3_95) == (s_3_94));
        // D s_3_97: lsl s_3_93 s_3_91
        let s_3_97: u64 = s_3_93 << s_3_91;
        // D s_3_98: or s_3_92 s_3_97
        let s_3_98: u64 = ((s_3_92) | (s_3_97));
        // D s_3_99: cmpl s_3_97
        let s_3_99: u64 = !s_3_97;
        // D s_3_100: and s_3_92 s_3_99
        let s_3_100: u64 = ((s_3_92) & (s_3_99));
        // D s_3_101: select s_3_96 s_3_98 s_3_100
        let s_3_101: u64 = if s_3_96 { s_3_98 } else { s_3_100 };
        // D s_3_102: cast trunc s_3_101 -> u8
        let s_3_102: bool = ((s_3_101) != 0);
        // D s_3_103: cast zx s_3_84 -> bv
        let s_3_103: Bits = Bits::new(s_3_84 as u128, 1u16);
        // D s_3_104: cast zx s_3_102 -> bv
        let s_3_104: Bits = Bits::new(s_3_102 as u128, 1u16);
        // D s_3_105: cmp-ne s_3_103 s_3_104
        let s_3_105: bool = ((s_3_103) != (s_3_104));
        // D s_3_106: write-var sub_r <= s_3_105
        fn_state.sub_r = s_3_105;
        // C s_3_107: const #1s : i
        let s_3_107: i128 = 1;
        // D s_3_108: read-var rot:u8
        let s_3_108: u8 = fn_state.rot;
        // D s_3_109: cast zx s_3_108 -> bv
        let s_3_109: Bits = Bits::new(s_3_108 as u128, 2u16);
        // C s_3_110: const #1u : u64
        let s_3_110: u64 = 1;
        // D s_3_111: bit-extract s_3_109 s_3_107 s_3_110
        let s_3_111: Bits = (Bits::new(
            ((s_3_109) >> (s_3_107)).value(),
            u16::try_from(s_3_110).unwrap(),
        ));
        // D s_3_112: cast reint s_3_111 -> u8
        let s_3_112: bool = ((s_3_111.value()) != 0);
        // C s_3_113: const #0s : i
        let s_3_113: i128 = 0;
        // C s_3_114: const #0u : u64
        let s_3_114: u64 = 0;
        // D s_3_115: cast zx s_3_112 -> u64
        let s_3_115: u64 = (s_3_112 as u64);
        // C s_3_116: const #1u : u64
        let s_3_116: u64 = 1;
        // D s_3_117: and s_3_115 s_3_116
        let s_3_117: u64 = ((s_3_115) & (s_3_116));
        // D s_3_118: cmp-eq s_3_117 s_3_116
        let s_3_118: bool = ((s_3_117) == (s_3_116));
        // D s_3_119: lsl s_3_115 s_3_113
        let s_3_119: u64 = s_3_115 << s_3_113;
        // D s_3_120: or s_3_114 s_3_119
        let s_3_120: u64 = ((s_3_114) | (s_3_119));
        // D s_3_121: cmpl s_3_119
        let s_3_121: u64 = !s_3_119;
        // D s_3_122: and s_3_114 s_3_121
        let s_3_122: u64 = ((s_3_114) & (s_3_121));
        // D s_3_123: select s_3_118 s_3_120 s_3_122
        let s_3_123: u64 = if s_3_118 { s_3_120 } else { s_3_122 };
        // D s_3_124: cast trunc s_3_123 -> u8
        let s_3_124: bool = ((s_3_123) != 0);
        // D s_3_125: cast zx s_3_124 -> bv
        let s_3_125: Bits = Bits::new(s_3_124 as u128, 1u16);
        // C s_3_126: const #1u : u8
        let s_3_126: bool = true;
        // C s_3_127: cast zx s_3_126 -> bv
        let s_3_127: Bits = Bits::new(s_3_126 as u128, 1u16);
        // D s_3_128: cmp-eq s_3_125 s_3_127
        let s_3_128: bool = ((s_3_125) == (s_3_127));
        // D s_3_129: write-var sub_i <= s_3_128
        fn_state.sub_i = s_3_128;
        // D s_3_130: read-var VL:i64
        let s_3_130: i64 = fn_state.VL;
        // C s_3_131: const #128s : i
        let s_3_131: i128 = 128;
        // D s_3_132: cast zx s_3_130 -> i
        let s_3_132: i128 = (i128::try_from(s_3_130).unwrap());
        // D s_3_133: cmp-eq s_3_132 s_3_131
        let s_3_133: bool = ((s_3_132) == (s_3_131));
        // D s_3_134: not s_3_133
        let s_3_134: bool = !s_3_133;
        // N s_3_135: branch s_3_134 b5 b4
        if s_3_134 {
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
        // D s_4_1: read-var da:i64
        let s_4_1: i64 = fn_state.da;
        // C s_4_2: const #16s : i64
        let s_4_2: i64 = 16;
        // D s_4_3: read-var index:i64
        let s_4_3: i64 = fn_state.index;
        // D s_4_4: read-var m:i64
        let s_4_4: i64 = fn_state.m;
        // D s_4_5: read-var n:i64
        let s_4_5: i64 = fn_state.n;
        // D s_4_6: read-var sel_a:i64
        let s_4_6: i64 = fn_state.sel_a;
        // D s_4_7: read-var sel_b:i64
        let s_4_7: i64 = fn_state.sel_b;
        // D s_4_8: read-var sub_i:u8
        let s_4_8: bool = fn_state.sub_i;
        // D s_4_9: read-var sub_r:u8
        let s_4_9: bool = fn_state.sub_r;
        // D s_4_10: call execute_SQRDCMLAH_Z_ZZZi_H(s_4_0, s_4_1, s_4_2, s_4_3, s_4_4, s_4_5, s_4_6, s_4_7, s_4_8, s_4_9)
        let s_4_10: () = execute_SQRDCMLAH_Z_ZZZi_H(
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
        );
        // N s_4_11: return
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
        // D s_6_1: read-var da:i64
        let s_6_1: i64 = fn_state.da;
        // C s_6_2: const #16s : i64
        let s_6_2: i64 = 16;
        // D s_6_3: read-var index:i64
        let s_6_3: i64 = fn_state.index;
        // D s_6_4: read-var m:i64
        let s_6_4: i64 = fn_state.m;
        // D s_6_5: read-var n:i64
        let s_6_5: i64 = fn_state.n;
        // D s_6_6: read-var sel_a:i64
        let s_6_6: i64 = fn_state.sel_a;
        // D s_6_7: read-var sel_b:i64
        let s_6_7: i64 = fn_state.sel_b;
        // D s_6_8: read-var sub_i:u8
        let s_6_8: bool = fn_state.sub_i;
        // D s_6_9: read-var sub_r:u8
        let s_6_9: bool = fn_state.sub_r;
        // D s_6_10: call execute_SQRDCMLAH_Z_ZZZi_H(s_6_0, s_6_1, s_6_2, s_6_3, s_6_4, s_6_5, s_6_6, s_6_7, s_6_8, s_6_9)
        let s_6_10: () = execute_SQRDCMLAH_Z_ZZZi_H(
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
        // D s_8_1: read-var da:i64
        let s_8_1: i64 = fn_state.da;
        // C s_8_2: const #16s : i64
        let s_8_2: i64 = 16;
        // D s_8_3: read-var index:i64
        let s_8_3: i64 = fn_state.index;
        // D s_8_4: read-var m:i64
        let s_8_4: i64 = fn_state.m;
        // D s_8_5: read-var n:i64
        let s_8_5: i64 = fn_state.n;
        // D s_8_6: read-var sel_a:i64
        let s_8_6: i64 = fn_state.sel_a;
        // D s_8_7: read-var sel_b:i64
        let s_8_7: i64 = fn_state.sel_b;
        // D s_8_8: read-var sub_i:u8
        let s_8_8: bool = fn_state.sub_i;
        // D s_8_9: read-var sub_r:u8
        let s_8_9: bool = fn_state.sub_r;
        // D s_8_10: call execute_SQRDCMLAH_Z_ZZZi_H(s_8_0, s_8_1, s_8_2, s_8_3, s_8_4, s_8_5, s_8_6, s_8_7, s_8_8, s_8_9)
        let s_8_10: () = execute_SQRDCMLAH_Z_ZZZi_H(
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
        // D s_10_1: read-var da:i64
        let s_10_1: i64 = fn_state.da;
        // C s_10_2: const #16s : i64
        let s_10_2: i64 = 16;
        // D s_10_3: read-var index:i64
        let s_10_3: i64 = fn_state.index;
        // D s_10_4: read-var m:i64
        let s_10_4: i64 = fn_state.m;
        // D s_10_5: read-var n:i64
        let s_10_5: i64 = fn_state.n;
        // D s_10_6: read-var sel_a:i64
        let s_10_6: i64 = fn_state.sel_a;
        // D s_10_7: read-var sel_b:i64
        let s_10_7: i64 = fn_state.sel_b;
        // D s_10_8: read-var sub_i:u8
        let s_10_8: bool = fn_state.sub_i;
        // D s_10_9: read-var sub_r:u8
        let s_10_9: bool = fn_state.sub_r;
        // D s_10_10: call execute_SQRDCMLAH_Z_ZZZi_H(s_10_0, s_10_1, s_10_2, s_10_3, s_10_4, s_10_5, s_10_6, s_10_7, s_10_8, s_10_9)
        let s_10_10: () = execute_SQRDCMLAH_Z_ZZZi_H(
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
        // D s_12_1: read-var da:i64
        let s_12_1: i64 = fn_state.da;
        // C s_12_2: const #16s : i64
        let s_12_2: i64 = 16;
        // D s_12_3: read-var index:i64
        let s_12_3: i64 = fn_state.index;
        // D s_12_4: read-var m:i64
        let s_12_4: i64 = fn_state.m;
        // D s_12_5: read-var n:i64
        let s_12_5: i64 = fn_state.n;
        // D s_12_6: read-var sel_a:i64
        let s_12_6: i64 = fn_state.sel_a;
        // D s_12_7: read-var sel_b:i64
        let s_12_7: i64 = fn_state.sel_b;
        // D s_12_8: read-var sub_i:u8
        let s_12_8: bool = fn_state.sub_i;
        // D s_12_9: read-var sub_r:u8
        let s_12_9: bool = fn_state.sub_r;
        // D s_12_10: call execute_SQRDCMLAH_Z_ZZZi_H(s_12_0, s_12_1, s_12_2, s_12_3, s_12_4, s_12_5, s_12_6, s_12_7, s_12_8, s_12_9)
        let s_12_10: () = execute_SQRDCMLAH_Z_ZZZi_H(
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
        );
        // N s_12_11: return
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
        // D s_15_3: write-var gs#210630 <= s_15_2
        fn_state.gs_210630 = s_15_2;
        // N s_15_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
