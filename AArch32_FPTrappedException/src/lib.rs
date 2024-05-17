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
use u_update_FPEXC_Type_TFV::*;
use u_update_FPEXC_Type_DEX::*;
use AArch32_TakeUndefInstrException::*;
use AArch32_GeneralExceptionsToAArch64::*;
use FPEXC_read::*;
use Mk_FPEXC_Type::*;
use AArch64_FPTrappedException::*;
use FPEXC_write::*;
use common::*;
pub fn AArch32_FPTrappedException<T: Tracer>(
    state: &mut State,
    tracer: &T,
    accumulated_exceptions: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_7477: ProductType700c18a878c5601b,
        ga_7486: ProductType700c18a878c5601b,
        ga_7495: ProductType700c18a878c5601b,
        accumulated_exceptions: u8,
    }
    let fn_state = FunctionState {
        accumulated_exceptions,
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
        // S s_0_1: call AArch32_GeneralExceptionsToAArch64(s_0_0)
        let s_0_1: bool = AArch32_GeneralExceptionsToAArch64(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b3 b1
        if s_0_1 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call FPEXC_read(s_2_0)
        let s_2_1: ProductType700c18a878c5601b = FPEXC_read(state, tracer, s_2_0);
        // C s_2_2: const #1u : u8
        let s_2_2: bool = true;
        // S s_2_3: call _update_FPEXC_Type_DEX(s_2_1, s_2_2)
        let s_2_3: ProductType700c18a878c5601b = u_update_FPEXC_Type_DEX(
            state,
            tracer,
            s_2_1,
            s_2_2,
        );
        // S s_2_4: call FPEXC_write(s_2_3)
        let s_2_4: () = FPEXC_write(state, tracer, s_2_3);
        // C s_2_5: const #() : ()
        let s_2_5: () = ();
        // S s_2_6: call FPEXC_read(s_2_5)
        let s_2_6: ProductType700c18a878c5601b = FPEXC_read(state, tracer, s_2_5);
        // C s_2_7: const #1u : u8
        let s_2_7: bool = true;
        // S s_2_8: call _update_FPEXC_Type_TFV(s_2_6, s_2_7)
        let s_2_8: ProductType700c18a878c5601b = u_update_FPEXC_Type_TFV(
            state,
            tracer,
            s_2_6,
            s_2_7,
        );
        // S s_2_9: call FPEXC_write(s_2_8)
        let s_2_9: () = FPEXC_write(state, tracer, s_2_8);
        // C s_2_10: const #() : ()
        let s_2_10: () = ();
        // S s_2_11: call FPEXC_read(s_2_10)
        let s_2_11: ProductType700c18a878c5601b = FPEXC_read(state, tracer, s_2_10);
        // D s_2_12: write-var ga#7477 <= s_2_11
        fn_state.ga_7477 = s_2_11;
        // D s_2_13: read-var ga#7477.0:struct
        let s_2_13: u32 = fn_state.ga_7477._0;
        // C s_2_14: const #7s : i
        let s_2_14: i128 = 7;
        // D s_2_15: read-var accumulated_exceptions:u8
        let s_2_15: u8 = fn_state.accumulated_exceptions;
        // D s_2_16: cast zx s_2_15 -> bv
        let s_2_16: Bits = Bits::new(s_2_15 as u128, 8u16);
        // C s_2_17: const #1u : u64
        let s_2_17: u64 = 1;
        // D s_2_18: bit-extract s_2_16 s_2_14 s_2_17
        let s_2_18: Bits = (Bits::new(
            ((s_2_16) >> (s_2_14)).value(),
            u16::try_from(s_2_17).unwrap(),
        ));
        // D s_2_19: cast reint s_2_18 -> u8
        let s_2_19: bool = ((s_2_18.value()) != 0);
        // C s_2_20: const #0s : i
        let s_2_20: i128 = 0;
        // C s_2_21: const #0u : u64
        let s_2_21: u64 = 0;
        // D s_2_22: cast zx s_2_19 -> u64
        let s_2_22: u64 = (s_2_19 as u64);
        // C s_2_23: const #1u : u64
        let s_2_23: u64 = 1;
        // D s_2_24: and s_2_22 s_2_23
        let s_2_24: u64 = ((s_2_22) & (s_2_23));
        // D s_2_25: cmp-eq s_2_24 s_2_23
        let s_2_25: bool = ((s_2_24) == (s_2_23));
        // D s_2_26: lsl s_2_22 s_2_20
        let s_2_26: u64 = s_2_22 << s_2_20;
        // D s_2_27: or s_2_21 s_2_26
        let s_2_27: u64 = ((s_2_21) | (s_2_26));
        // D s_2_28: cmpl s_2_26
        let s_2_28: u64 = !s_2_26;
        // D s_2_29: and s_2_21 s_2_28
        let s_2_29: u64 = ((s_2_21) & (s_2_28));
        // D s_2_30: select s_2_25 s_2_27 s_2_29
        let s_2_30: u64 = if s_2_25 { s_2_27 } else { s_2_29 };
        // D s_2_31: cast trunc s_2_30 -> u8
        let s_2_31: bool = ((s_2_30) != 0);
        // C s_2_32: const #0s : i
        let s_2_32: i128 = 0;
        // D s_2_33: read-var accumulated_exceptions:u8
        let s_2_33: u8 = fn_state.accumulated_exceptions;
        // D s_2_34: cast zx s_2_33 -> bv
        let s_2_34: Bits = Bits::new(s_2_33 as u128, 8u16);
        // C s_2_35: const #1s : i64
        let s_2_35: i64 = 1;
        // C s_2_36: cast zx s_2_35 -> i
        let s_2_36: i128 = (i128::try_from(s_2_35).unwrap());
        // C s_2_37: const #4s : i
        let s_2_37: i128 = 4;
        // C s_2_38: add s_2_37 s_2_36
        let s_2_38: i128 = (s_2_37 + s_2_36);
        // D s_2_39: bit-extract s_2_34 s_2_32 s_2_38
        let s_2_39: Bits = (Bits::new(
            ((s_2_34) >> (s_2_32)).value(),
            u16::try_from(s_2_38).unwrap(),
        ));
        // D s_2_40: cast reint s_2_39 -> u8
        let s_2_40: u8 = (s_2_39.value() as u8);
        // D s_2_41: cast zx s_2_31 -> bv
        let s_2_41: Bits = Bits::new(s_2_31 as u128, 1u16);
        // D s_2_42: cast zx s_2_40 -> bv
        let s_2_42: Bits = Bits::new(s_2_40 as u128, 5u16);
        // D s_2_43: cast reint s_2_41 -> u128
        let s_2_43: u128 = (s_2_41.value() as u128);
        // D s_2_44: size-of s_2_41
        let s_2_44: u16 = s_2_41.length();
        // D s_2_45: cast reint s_2_42 -> u128
        let s_2_45: u128 = (s_2_42.value() as u128);
        // D s_2_46: size-of s_2_42
        let s_2_46: u16 = s_2_42.length();
        // D s_2_47: lsl s_2_43 s_2_46
        let s_2_47: u128 = s_2_43 << s_2_46;
        // D s_2_48: or s_2_47 s_2_45
        let s_2_48: u128 = ((s_2_47) | (s_2_45));
        // D s_2_49: add s_2_44 s_2_46
        let s_2_49: u16 = (s_2_44 + s_2_46);
        // D s_2_50: create-bits s_2_48 s_2_49
        let s_2_50: Bits = Bits::new(s_2_48, s_2_49);
        // D s_2_51: cast reint s_2_50 -> u8
        let s_2_51: u8 = (s_2_50.value() as u8);
        // C s_2_52: const #5s : i
        let s_2_52: i128 = 5;
        // C s_2_53: const #1s : i
        let s_2_53: i128 = 1;
        // D s_2_54: cast zx s_2_51 -> bv
        let s_2_54: Bits = Bits::new(s_2_51 as u128, 6u16);
        // D s_2_55: bit-extract s_2_54 s_2_52 s_2_53
        let s_2_55: Bits = (Bits::new(
            ((s_2_54) >> (s_2_52)).value(),
            u16::try_from(s_2_53).unwrap(),
        ));
        // D s_2_56: cast reint s_2_55 -> u8
        let s_2_56: bool = ((s_2_55.value()) != 0);
        // C s_2_57: const #7s : i
        let s_2_57: i128 = 7;
        // D s_2_58: cast zx s_2_13 -> bv
        let s_2_58: Bits = Bits::new(s_2_13 as u128, 32u16);
        // D s_2_59: cast zx s_2_56 -> bv
        let s_2_59: Bits = Bits::new(s_2_56 as u128, 1u16);
        // C s_2_60: const #0s : i
        let s_2_60: i128 = 0;
        // C s_2_61: const #1u : u64
        let s_2_61: u64 = 1;
        // C s_2_62: cast zx s_2_61 -> bv
        let s_2_62: Bits = Bits::new(s_2_61 as u128, 64u16);
        // C s_2_63: lsl s_2_62 s_2_60
        let s_2_63: Bits = s_2_62 << s_2_60;
        // C s_2_64: sub s_2_63 s_2_62
        let s_2_64: Bits = ((s_2_63) - (s_2_62));
        // D s_2_65: and s_2_59 s_2_64
        let s_2_65: Bits = ((s_2_59) & (s_2_64));
        // D s_2_66: lsl s_2_65 s_2_57
        let s_2_66: Bits = s_2_65 << s_2_57;
        // C s_2_67: lsl s_2_64 s_2_57
        let s_2_67: Bits = s_2_64 << s_2_57;
        // C s_2_68: cmpl s_2_67
        let s_2_68: Bits = !s_2_67;
        // D s_2_69: and s_2_58 s_2_68
        let s_2_69: Bits = ((s_2_58) & (s_2_68));
        // D s_2_70: or s_2_69 s_2_66
        let s_2_70: Bits = ((s_2_69) | (s_2_66));
        // D s_2_71: cast reint s_2_70 -> u32
        let s_2_71: u32 = (s_2_70.value() as u32);
        // D s_2_72: call Mk_FPEXC_Type(s_2_71)
        let s_2_72: ProductType700c18a878c5601b = Mk_FPEXC_Type(state, tracer, s_2_71);
        // D s_2_73: call FPEXC_write(s_2_72)
        let s_2_73: () = FPEXC_write(state, tracer, s_2_72);
        // C s_2_74: const #() : ()
        let s_2_74: () = ();
        // S s_2_75: call FPEXC_read(s_2_74)
        let s_2_75: ProductType700c18a878c5601b = FPEXC_read(state, tracer, s_2_74);
        // D s_2_76: write-var ga#7486 <= s_2_75
        fn_state.ga_7486 = s_2_75;
        // D s_2_77: read-var ga#7486.0:struct
        let s_2_77: u32 = fn_state.ga_7486._0;
        // C s_2_78: const #7s : i
        let s_2_78: i128 = 7;
        // D s_2_79: read-var accumulated_exceptions:u8
        let s_2_79: u8 = fn_state.accumulated_exceptions;
        // D s_2_80: cast zx s_2_79 -> bv
        let s_2_80: Bits = Bits::new(s_2_79 as u128, 8u16);
        // C s_2_81: const #1u : u64
        let s_2_81: u64 = 1;
        // D s_2_82: bit-extract s_2_80 s_2_78 s_2_81
        let s_2_82: Bits = (Bits::new(
            ((s_2_80) >> (s_2_78)).value(),
            u16::try_from(s_2_81).unwrap(),
        ));
        // D s_2_83: cast reint s_2_82 -> u8
        let s_2_83: bool = ((s_2_82.value()) != 0);
        // C s_2_84: const #0s : i
        let s_2_84: i128 = 0;
        // C s_2_85: const #0u : u64
        let s_2_85: u64 = 0;
        // D s_2_86: cast zx s_2_83 -> u64
        let s_2_86: u64 = (s_2_83 as u64);
        // C s_2_87: const #1u : u64
        let s_2_87: u64 = 1;
        // D s_2_88: and s_2_86 s_2_87
        let s_2_88: u64 = ((s_2_86) & (s_2_87));
        // D s_2_89: cmp-eq s_2_88 s_2_87
        let s_2_89: bool = ((s_2_88) == (s_2_87));
        // D s_2_90: lsl s_2_86 s_2_84
        let s_2_90: u64 = s_2_86 << s_2_84;
        // D s_2_91: or s_2_85 s_2_90
        let s_2_91: u64 = ((s_2_85) | (s_2_90));
        // D s_2_92: cmpl s_2_90
        let s_2_92: u64 = !s_2_90;
        // D s_2_93: and s_2_85 s_2_92
        let s_2_93: u64 = ((s_2_85) & (s_2_92));
        // D s_2_94: select s_2_89 s_2_91 s_2_93
        let s_2_94: u64 = if s_2_89 { s_2_91 } else { s_2_93 };
        // D s_2_95: cast trunc s_2_94 -> u8
        let s_2_95: bool = ((s_2_94) != 0);
        // C s_2_96: const #0s : i
        let s_2_96: i128 = 0;
        // D s_2_97: read-var accumulated_exceptions:u8
        let s_2_97: u8 = fn_state.accumulated_exceptions;
        // D s_2_98: cast zx s_2_97 -> bv
        let s_2_98: Bits = Bits::new(s_2_97 as u128, 8u16);
        // C s_2_99: const #1s : i64
        let s_2_99: i64 = 1;
        // C s_2_100: cast zx s_2_99 -> i
        let s_2_100: i128 = (i128::try_from(s_2_99).unwrap());
        // C s_2_101: const #4s : i
        let s_2_101: i128 = 4;
        // C s_2_102: add s_2_101 s_2_100
        let s_2_102: i128 = (s_2_101 + s_2_100);
        // D s_2_103: bit-extract s_2_98 s_2_96 s_2_102
        let s_2_103: Bits = (Bits::new(
            ((s_2_98) >> (s_2_96)).value(),
            u16::try_from(s_2_102).unwrap(),
        ));
        // D s_2_104: cast reint s_2_103 -> u8
        let s_2_104: u8 = (s_2_103.value() as u8);
        // D s_2_105: cast zx s_2_95 -> bv
        let s_2_105: Bits = Bits::new(s_2_95 as u128, 1u16);
        // D s_2_106: cast zx s_2_104 -> bv
        let s_2_106: Bits = Bits::new(s_2_104 as u128, 5u16);
        // D s_2_107: cast reint s_2_105 -> u128
        let s_2_107: u128 = (s_2_105.value() as u128);
        // D s_2_108: size-of s_2_105
        let s_2_108: u16 = s_2_105.length();
        // D s_2_109: cast reint s_2_106 -> u128
        let s_2_109: u128 = (s_2_106.value() as u128);
        // D s_2_110: size-of s_2_106
        let s_2_110: u16 = s_2_106.length();
        // D s_2_111: lsl s_2_107 s_2_110
        let s_2_111: u128 = s_2_107 << s_2_110;
        // D s_2_112: or s_2_111 s_2_109
        let s_2_112: u128 = ((s_2_111) | (s_2_109));
        // D s_2_113: add s_2_108 s_2_110
        let s_2_113: u16 = (s_2_108 + s_2_110);
        // D s_2_114: create-bits s_2_112 s_2_113
        let s_2_114: Bits = Bits::new(s_2_112, s_2_113);
        // D s_2_115: cast reint s_2_114 -> u8
        let s_2_115: u8 = (s_2_114.value() as u8);
        // C s_2_116: const #0s : i
        let s_2_116: i128 = 0;
        // C s_2_117: const #5s : i
        let s_2_117: i128 = 5;
        // D s_2_118: cast zx s_2_115 -> bv
        let s_2_118: Bits = Bits::new(s_2_115 as u128, 6u16);
        // D s_2_119: bit-extract s_2_118 s_2_116 s_2_117
        let s_2_119: Bits = (Bits::new(
            ((s_2_118) >> (s_2_116)).value(),
            u16::try_from(s_2_117).unwrap(),
        ));
        // D s_2_120: cast reint s_2_119 -> u8
        let s_2_120: u8 = (s_2_119.value() as u8);
        // C s_2_121: const #0s : i
        let s_2_121: i128 = 0;
        // D s_2_122: cast zx s_2_77 -> bv
        let s_2_122: Bits = Bits::new(s_2_77 as u128, 32u16);
        // D s_2_123: cast zx s_2_120 -> bv
        let s_2_123: Bits = Bits::new(s_2_120 as u128, 5u16);
        // C s_2_124: const #4s : i
        let s_2_124: i128 = 4;
        // C s_2_125: const #1u : u64
        let s_2_125: u64 = 1;
        // C s_2_126: cast zx s_2_125 -> bv
        let s_2_126: Bits = Bits::new(s_2_125 as u128, 64u16);
        // C s_2_127: lsl s_2_126 s_2_124
        let s_2_127: Bits = s_2_126 << s_2_124;
        // C s_2_128: sub s_2_127 s_2_126
        let s_2_128: Bits = ((s_2_127) - (s_2_126));
        // D s_2_129: and s_2_123 s_2_128
        let s_2_129: Bits = ((s_2_123) & (s_2_128));
        // D s_2_130: lsl s_2_129 s_2_121
        let s_2_130: Bits = s_2_129 << s_2_121;
        // C s_2_131: lsl s_2_128 s_2_121
        let s_2_131: Bits = s_2_128 << s_2_121;
        // C s_2_132: cmpl s_2_131
        let s_2_132: Bits = !s_2_131;
        // D s_2_133: and s_2_122 s_2_132
        let s_2_133: Bits = ((s_2_122) & (s_2_132));
        // D s_2_134: or s_2_133 s_2_130
        let s_2_134: Bits = ((s_2_133) | (s_2_130));
        // D s_2_135: cast reint s_2_134 -> u32
        let s_2_135: u32 = (s_2_134.value() as u32);
        // D s_2_136: call Mk_FPEXC_Type(s_2_135)
        let s_2_136: ProductType700c18a878c5601b = Mk_FPEXC_Type(state, tracer, s_2_135);
        // D s_2_137: call FPEXC_write(s_2_136)
        let s_2_137: () = FPEXC_write(state, tracer, s_2_136);
        // C s_2_138: const #() : ()
        let s_2_138: () = ();
        // S s_2_139: call FPEXC_read(s_2_138)
        let s_2_139: ProductType700c18a878c5601b = FPEXC_read(state, tracer, s_2_138);
        // D s_2_140: write-var ga#7495 <= s_2_139
        fn_state.ga_7495 = s_2_139;
        // D s_2_141: read-var ga#7495.0:struct
        let s_2_141: u32 = fn_state.ga_7495._0;
        // C s_2_142: const #8s : i
        let s_2_142: i128 = 8;
        // D s_2_143: cast zx s_2_141 -> bv
        let s_2_143: Bits = Bits::new(s_2_141 as u128, 32u16);
        // C s_2_144: const #7u : u8
        let s_2_144: u8 = 7;
        // C s_2_145: cast zx s_2_144 -> bv
        let s_2_145: Bits = Bits::new(s_2_144 as u128, 3u16);
        // C s_2_146: const #2s : i
        let s_2_146: i128 = 2;
        // C s_2_147: const #1u : u64
        let s_2_147: u64 = 1;
        // C s_2_148: cast zx s_2_147 -> bv
        let s_2_148: Bits = Bits::new(s_2_147 as u128, 64u16);
        // C s_2_149: lsl s_2_148 s_2_146
        let s_2_149: Bits = s_2_148 << s_2_146;
        // C s_2_150: sub s_2_149 s_2_148
        let s_2_150: Bits = ((s_2_149) - (s_2_148));
        // C s_2_151: and s_2_145 s_2_150
        let s_2_151: Bits = ((s_2_145) & (s_2_150));
        // C s_2_152: lsl s_2_151 s_2_142
        let s_2_152: Bits = s_2_151 << s_2_142;
        // C s_2_153: lsl s_2_150 s_2_142
        let s_2_153: Bits = s_2_150 << s_2_142;
        // C s_2_154: cmpl s_2_153
        let s_2_154: Bits = !s_2_153;
        // D s_2_155: and s_2_143 s_2_154
        let s_2_155: Bits = ((s_2_143) & (s_2_154));
        // D s_2_156: or s_2_155 s_2_152
        let s_2_156: Bits = ((s_2_155) | (s_2_152));
        // D s_2_157: cast reint s_2_156 -> u32
        let s_2_157: u32 = (s_2_156.value() as u32);
        // D s_2_158: call Mk_FPEXC_Type(s_2_157)
        let s_2_158: ProductType700c18a878c5601b = Mk_FPEXC_Type(state, tracer, s_2_157);
        // D s_2_159: call FPEXC_write(s_2_158)
        let s_2_159: () = FPEXC_write(state, tracer, s_2_158);
        // C s_2_160: const #() : ()
        let s_2_160: () = ();
        // S s_2_161: call AArch32_TakeUndefInstrException(s_2_160)
        let s_2_161: () = AArch32_TakeUndefInstrException(state, tracer, s_2_160);
        // N s_2_162: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: read-var accumulated_exceptions:u8
        let s_3_1: u8 = fn_state.accumulated_exceptions;
        // D s_3_2: call AArch64_FPTrappedException(s_3_0, s_3_1)
        let s_3_2: () = AArch64_FPTrappedException(state, tracer, s_3_0, s_3_1);
        // N s_3_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
