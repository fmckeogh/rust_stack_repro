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
use u_update_DBGDSCRext_Type_TXfull::*;
use DBGDSCRext_read::*;
use u_get_DBGDSCRext_Type_TXfull::*;
use DBGDSCRext_write::*;
use u_get_DBGDSCRext_Type_HDE::*;
use u_update_DBGDSCRext_Type_TDA::*;
use u_get_DBGOSLSR_Type_OSLK::*;
use u_update_DBGDSCRext_Type_INTdis::*;
use Mk_DBGDSCRext_Type::*;
use u_get_DBGDSCRext_Type_RXfull::*;
use u_get_DBGDSCRext_Type_TDA::*;
use u_update_DBGDSCRext_Type_HDE::*;
use u_get_DBGDSCRext_Type_SC2::*;
use u_update_DBGDSCRext_Type_ERR::*;
use u_update_DBGDSCRext_Type_TXU::*;
use u_get_DBGDSCRext_Type_RXO::*;
use u_get_DBGDSCRext_Type_TFO::*;
use u_get_DBGDSCRext_Type_TXU::*;
use u_get_DBGDSCRext_Type_ERR::*;
use u_update_DBGDSCRext_Type_RXO::*;
use DBGOSLSR_read::*;
use u_update_DBGDSCRext_Type_SC2::*;
use u_get_DBGDSCRext_Type_INTdis::*;
use u_update_DBGDSCRext_Type_RXfull::*;
use u_update_DBGDSCRext_Type_TFO::*;
use common::*;
pub fn u__set_DBGDSCRext<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType700c18a878c5601b,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductType700c18a878c5601b,
        ga_28233: ProductType700c18a878c5601b,
        ga_28209: ProductType700c18a878c5601b,
        ga_28215: ProductType700c18a878c5601b,
        ga_28227: ProductType700c18a878c5601b,
        ga_28221: ProductType700c18a878c5601b,
        ga_28203: ProductType700c18a878c5601b,
        value_name: ProductType700c18a878c5601b,
    }
    let fn_state = FunctionState {
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var value_name:struct
        let s_0_0: ProductType700c18a878c5601b = fn_state.value_name;
        // D s_0_1: write-var r <= s_0_0
        fn_state.r = s_0_0;
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call DBGDSCRext_read(s_0_2)
        let s_0_3: ProductType700c18a878c5601b = DBGDSCRext_read(state, tracer, s_0_2);
        // D s_0_4: write-var ga#28203 <= s_0_3
        fn_state.ga_28203 = s_0_3;
        // D s_0_5: read-var ga#28203.0:struct
        let s_0_5: u32 = fn_state.ga_28203._0;
        // D s_0_6: read-var r.0:struct
        let s_0_6: u32 = fn_state.r._0;
        // C s_0_7: const #0s : i
        let s_0_7: i128 = 0;
        // C s_0_8: const #6s : i
        let s_0_8: i128 = 6;
        // D s_0_9: cast zx s_0_6 -> bv
        let s_0_9: Bits = Bits::new(s_0_6 as u128, 32u16);
        // D s_0_10: bit-extract s_0_9 s_0_7 s_0_8
        let s_0_10: Bits = (Bits::new(
            ((s_0_9) >> (s_0_7)).value(),
            u16::try_from(s_0_8).unwrap(),
        ));
        // D s_0_11: cast reint s_0_10 -> u8
        let s_0_11: u8 = (s_0_10.value() as u8);
        // C s_0_12: const #6s : i
        let s_0_12: i128 = 6;
        // C s_0_13: const #0s : i
        let s_0_13: i128 = 0;
        // D s_0_14: cast zx s_0_5 -> bv
        let s_0_14: Bits = Bits::new(s_0_5 as u128, 32u16);
        // D s_0_15: cast zx s_0_11 -> bv
        let s_0_15: Bits = Bits::new(s_0_11 as u128, 6u16);
        // C s_0_16: const #1u : u64
        let s_0_16: u64 = 1;
        // C s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 64u16);
        // C s_0_18: lsl s_0_17 s_0_12
        let s_0_18: Bits = s_0_17 << s_0_12;
        // C s_0_19: sub s_0_18 s_0_17
        let s_0_19: Bits = ((s_0_18) - (s_0_17));
        // D s_0_20: and s_0_15 s_0_19
        let s_0_20: Bits = ((s_0_15) & (s_0_19));
        // D s_0_21: lsl s_0_20 s_0_13
        let s_0_21: Bits = s_0_20 << s_0_13;
        // C s_0_22: lsl s_0_19 s_0_13
        let s_0_22: Bits = s_0_19 << s_0_13;
        // C s_0_23: cmpl s_0_22
        let s_0_23: Bits = !s_0_22;
        // D s_0_24: and s_0_14 s_0_23
        let s_0_24: Bits = ((s_0_14) & (s_0_23));
        // D s_0_25: or s_0_24 s_0_21
        let s_0_25: Bits = ((s_0_24) | (s_0_21));
        // D s_0_26: cast reint s_0_25 -> u32
        let s_0_26: u32 = (s_0_25.value() as u32);
        // D s_0_27: call Mk_DBGDSCRext_Type(s_0_26)
        let s_0_27: ProductType700c18a878c5601b = Mk_DBGDSCRext_Type(
            state,
            tracer,
            s_0_26,
        );
        // D s_0_28: call DBGDSCRext_write(s_0_27)
        let s_0_28: () = DBGDSCRext_write(state, tracer, s_0_27);
        // C s_0_29: const #() : ()
        let s_0_29: () = ();
        // S s_0_30: call DBGDSCRext_read(s_0_29)
        let s_0_30: ProductType700c18a878c5601b = DBGDSCRext_read(state, tracer, s_0_29);
        // D s_0_31: write-var ga#28209 <= s_0_30
        fn_state.ga_28209 = s_0_30;
        // D s_0_32: read-var ga#28209.0:struct
        let s_0_32: u32 = fn_state.ga_28209._0;
        // D s_0_33: read-var r.0:struct
        let s_0_33: u32 = fn_state.r._0;
        // C s_0_34: const #7s : i
        let s_0_34: i128 = 7;
        // C s_0_35: const #7s : i
        let s_0_35: i128 = 7;
        // D s_0_36: cast zx s_0_33 -> bv
        let s_0_36: Bits = Bits::new(s_0_33 as u128, 32u16);
        // D s_0_37: bit-extract s_0_36 s_0_34 s_0_35
        let s_0_37: Bits = (Bits::new(
            ((s_0_36) >> (s_0_34)).value(),
            u16::try_from(s_0_35).unwrap(),
        ));
        // D s_0_38: cast reint s_0_37 -> u8
        let s_0_38: u8 = (s_0_37.value() as u8);
        // C s_0_39: const #7s : i
        let s_0_39: i128 = 7;
        // C s_0_40: const #7s : i
        let s_0_40: i128 = 7;
        // D s_0_41: cast zx s_0_32 -> bv
        let s_0_41: Bits = Bits::new(s_0_32 as u128, 32u16);
        // D s_0_42: cast zx s_0_38 -> bv
        let s_0_42: Bits = Bits::new(s_0_38 as u128, 7u16);
        // C s_0_43: const #1u : u64
        let s_0_43: u64 = 1;
        // C s_0_44: cast zx s_0_43 -> bv
        let s_0_44: Bits = Bits::new(s_0_43 as u128, 64u16);
        // C s_0_45: lsl s_0_44 s_0_39
        let s_0_45: Bits = s_0_44 << s_0_39;
        // C s_0_46: sub s_0_45 s_0_44
        let s_0_46: Bits = ((s_0_45) - (s_0_44));
        // D s_0_47: and s_0_42 s_0_46
        let s_0_47: Bits = ((s_0_42) & (s_0_46));
        // D s_0_48: lsl s_0_47 s_0_40
        let s_0_48: Bits = s_0_47 << s_0_40;
        // C s_0_49: lsl s_0_46 s_0_40
        let s_0_49: Bits = s_0_46 << s_0_40;
        // C s_0_50: cmpl s_0_49
        let s_0_50: Bits = !s_0_49;
        // D s_0_51: and s_0_41 s_0_50
        let s_0_51: Bits = ((s_0_41) & (s_0_50));
        // D s_0_52: or s_0_51 s_0_48
        let s_0_52: Bits = ((s_0_51) | (s_0_48));
        // D s_0_53: cast reint s_0_52 -> u32
        let s_0_53: u32 = (s_0_52.value() as u32);
        // D s_0_54: call Mk_DBGDSCRext_Type(s_0_53)
        let s_0_54: ProductType700c18a878c5601b = Mk_DBGDSCRext_Type(
            state,
            tracer,
            s_0_53,
        );
        // D s_0_55: call DBGDSCRext_write(s_0_54)
        let s_0_55: () = DBGDSCRext_write(state, tracer, s_0_54);
        // C s_0_56: const #() : ()
        let s_0_56: () = ();
        // S s_0_57: call DBGDSCRext_read(s_0_56)
        let s_0_57: ProductType700c18a878c5601b = DBGDSCRext_read(state, tracer, s_0_56);
        // D s_0_58: write-var ga#28215 <= s_0_57
        fn_state.ga_28215 = s_0_57;
        // D s_0_59: read-var ga#28215.0:struct
        let s_0_59: u32 = fn_state.ga_28215._0;
        // D s_0_60: read-var r.0:struct
        let s_0_60: u32 = fn_state.r._0;
        // C s_0_61: const #15s : i
        let s_0_61: i128 = 15;
        // C s_0_62: const #1s : i
        let s_0_62: i128 = 1;
        // D s_0_63: cast zx s_0_60 -> bv
        let s_0_63: Bits = Bits::new(s_0_60 as u128, 32u16);
        // D s_0_64: bit-extract s_0_63 s_0_61 s_0_62
        let s_0_64: Bits = (Bits::new(
            ((s_0_63) >> (s_0_61)).value(),
            u16::try_from(s_0_62).unwrap(),
        ));
        // D s_0_65: cast reint s_0_64 -> u8
        let s_0_65: bool = ((s_0_64.value()) != 0);
        // C s_0_66: const #1s : i
        let s_0_66: i128 = 1;
        // C s_0_67: const #15s : i
        let s_0_67: i128 = 15;
        // D s_0_68: cast zx s_0_59 -> bv
        let s_0_68: Bits = Bits::new(s_0_59 as u128, 32u16);
        // D s_0_69: cast zx s_0_65 -> bv
        let s_0_69: Bits = Bits::new(s_0_65 as u128, 1u16);
        // C s_0_70: const #1u : u64
        let s_0_70: u64 = 1;
        // C s_0_71: cast zx s_0_70 -> bv
        let s_0_71: Bits = Bits::new(s_0_70 as u128, 64u16);
        // C s_0_72: lsl s_0_71 s_0_66
        let s_0_72: Bits = s_0_71 << s_0_66;
        // C s_0_73: sub s_0_72 s_0_71
        let s_0_73: Bits = ((s_0_72) - (s_0_71));
        // D s_0_74: and s_0_69 s_0_73
        let s_0_74: Bits = ((s_0_69) & (s_0_73));
        // D s_0_75: lsl s_0_74 s_0_67
        let s_0_75: Bits = s_0_74 << s_0_67;
        // C s_0_76: lsl s_0_73 s_0_67
        let s_0_76: Bits = s_0_73 << s_0_67;
        // C s_0_77: cmpl s_0_76
        let s_0_77: Bits = !s_0_76;
        // D s_0_78: and s_0_68 s_0_77
        let s_0_78: Bits = ((s_0_68) & (s_0_77));
        // D s_0_79: or s_0_78 s_0_75
        let s_0_79: Bits = ((s_0_78) | (s_0_75));
        // D s_0_80: cast reint s_0_79 -> u32
        let s_0_80: u32 = (s_0_79.value() as u32);
        // D s_0_81: call Mk_DBGDSCRext_Type(s_0_80)
        let s_0_81: ProductType700c18a878c5601b = Mk_DBGDSCRext_Type(
            state,
            tracer,
            s_0_80,
        );
        // D s_0_82: call DBGDSCRext_write(s_0_81)
        let s_0_82: () = DBGDSCRext_write(state, tracer, s_0_81);
        // C s_0_83: const #() : ()
        let s_0_83: () = ();
        // S s_0_84: call DBGDSCRext_read(s_0_83)
        let s_0_84: ProductType700c18a878c5601b = DBGDSCRext_read(state, tracer, s_0_83);
        // D s_0_85: write-var ga#28221 <= s_0_84
        fn_state.ga_28221 = s_0_84;
        // D s_0_86: read-var ga#28221.0:struct
        let s_0_86: u32 = fn_state.ga_28221._0;
        // D s_0_87: read-var r.0:struct
        let s_0_87: u32 = fn_state.r._0;
        // C s_0_88: const #20s : i
        let s_0_88: i128 = 20;
        // C s_0_89: const #1s : i
        let s_0_89: i128 = 1;
        // D s_0_90: cast zx s_0_87 -> bv
        let s_0_90: Bits = Bits::new(s_0_87 as u128, 32u16);
        // D s_0_91: bit-extract s_0_90 s_0_88 s_0_89
        let s_0_91: Bits = (Bits::new(
            ((s_0_90) >> (s_0_88)).value(),
            u16::try_from(s_0_89).unwrap(),
        ));
        // D s_0_92: cast reint s_0_91 -> u8
        let s_0_92: bool = ((s_0_91.value()) != 0);
        // C s_0_93: const #1s : i
        let s_0_93: i128 = 1;
        // C s_0_94: const #20s : i
        let s_0_94: i128 = 20;
        // D s_0_95: cast zx s_0_86 -> bv
        let s_0_95: Bits = Bits::new(s_0_86 as u128, 32u16);
        // D s_0_96: cast zx s_0_92 -> bv
        let s_0_96: Bits = Bits::new(s_0_92 as u128, 1u16);
        // C s_0_97: const #1u : u64
        let s_0_97: u64 = 1;
        // C s_0_98: cast zx s_0_97 -> bv
        let s_0_98: Bits = Bits::new(s_0_97 as u128, 64u16);
        // C s_0_99: lsl s_0_98 s_0_93
        let s_0_99: Bits = s_0_98 << s_0_93;
        // C s_0_100: sub s_0_99 s_0_98
        let s_0_100: Bits = ((s_0_99) - (s_0_98));
        // D s_0_101: and s_0_96 s_0_100
        let s_0_101: Bits = ((s_0_96) & (s_0_100));
        // D s_0_102: lsl s_0_101 s_0_94
        let s_0_102: Bits = s_0_101 << s_0_94;
        // C s_0_103: lsl s_0_100 s_0_94
        let s_0_103: Bits = s_0_100 << s_0_94;
        // C s_0_104: cmpl s_0_103
        let s_0_104: Bits = !s_0_103;
        // D s_0_105: and s_0_95 s_0_104
        let s_0_105: Bits = ((s_0_95) & (s_0_104));
        // D s_0_106: or s_0_105 s_0_102
        let s_0_106: Bits = ((s_0_105) | (s_0_102));
        // D s_0_107: cast reint s_0_106 -> u32
        let s_0_107: u32 = (s_0_106.value() as u32);
        // D s_0_108: call Mk_DBGDSCRext_Type(s_0_107)
        let s_0_108: ProductType700c18a878c5601b = Mk_DBGDSCRext_Type(
            state,
            tracer,
            s_0_107,
        );
        // D s_0_109: call DBGDSCRext_write(s_0_108)
        let s_0_109: () = DBGDSCRext_write(state, tracer, s_0_108);
        // C s_0_110: const #() : ()
        let s_0_110: () = ();
        // S s_0_111: call DBGDSCRext_read(s_0_110)
        let s_0_111: ProductType700c18a878c5601b = DBGDSCRext_read(
            state,
            tracer,
            s_0_110,
        );
        // D s_0_112: write-var ga#28227 <= s_0_111
        fn_state.ga_28227 = s_0_111;
        // D s_0_113: read-var ga#28227.0:struct
        let s_0_113: u32 = fn_state.ga_28227._0;
        // D s_0_114: read-var r.0:struct
        let s_0_114: u32 = fn_state.r._0;
        // C s_0_115: const #24s : i
        let s_0_115: i128 = 24;
        // C s_0_116: const #2s : i
        let s_0_116: i128 = 2;
        // D s_0_117: cast zx s_0_114 -> bv
        let s_0_117: Bits = Bits::new(s_0_114 as u128, 32u16);
        // D s_0_118: bit-extract s_0_117 s_0_115 s_0_116
        let s_0_118: Bits = (Bits::new(
            ((s_0_117) >> (s_0_115)).value(),
            u16::try_from(s_0_116).unwrap(),
        ));
        // D s_0_119: cast reint s_0_118 -> u8
        let s_0_119: u8 = (s_0_118.value() as u8);
        // C s_0_120: const #2s : i
        let s_0_120: i128 = 2;
        // C s_0_121: const #24s : i
        let s_0_121: i128 = 24;
        // D s_0_122: cast zx s_0_113 -> bv
        let s_0_122: Bits = Bits::new(s_0_113 as u128, 32u16);
        // D s_0_123: cast zx s_0_119 -> bv
        let s_0_123: Bits = Bits::new(s_0_119 as u128, 2u16);
        // C s_0_124: const #1u : u64
        let s_0_124: u64 = 1;
        // C s_0_125: cast zx s_0_124 -> bv
        let s_0_125: Bits = Bits::new(s_0_124 as u128, 64u16);
        // C s_0_126: lsl s_0_125 s_0_120
        let s_0_126: Bits = s_0_125 << s_0_120;
        // C s_0_127: sub s_0_126 s_0_125
        let s_0_127: Bits = ((s_0_126) - (s_0_125));
        // D s_0_128: and s_0_123 s_0_127
        let s_0_128: Bits = ((s_0_123) & (s_0_127));
        // D s_0_129: lsl s_0_128 s_0_121
        let s_0_129: Bits = s_0_128 << s_0_121;
        // C s_0_130: lsl s_0_127 s_0_121
        let s_0_130: Bits = s_0_127 << s_0_121;
        // C s_0_131: cmpl s_0_130
        let s_0_131: Bits = !s_0_130;
        // D s_0_132: and s_0_122 s_0_131
        let s_0_132: Bits = ((s_0_122) & (s_0_131));
        // D s_0_133: or s_0_132 s_0_129
        let s_0_133: Bits = ((s_0_132) | (s_0_129));
        // D s_0_134: cast reint s_0_133 -> u32
        let s_0_134: u32 = (s_0_133.value() as u32);
        // D s_0_135: call Mk_DBGDSCRext_Type(s_0_134)
        let s_0_135: ProductType700c18a878c5601b = Mk_DBGDSCRext_Type(
            state,
            tracer,
            s_0_134,
        );
        // D s_0_136: call DBGDSCRext_write(s_0_135)
        let s_0_136: () = DBGDSCRext_write(state, tracer, s_0_135);
        // C s_0_137: const #() : ()
        let s_0_137: () = ();
        // S s_0_138: call DBGDSCRext_read(s_0_137)
        let s_0_138: ProductType700c18a878c5601b = DBGDSCRext_read(
            state,
            tracer,
            s_0_137,
        );
        // D s_0_139: write-var ga#28233 <= s_0_138
        fn_state.ga_28233 = s_0_138;
        // D s_0_140: read-var ga#28233.0:struct
        let s_0_140: u32 = fn_state.ga_28233._0;
        // D s_0_141: read-var r.0:struct
        let s_0_141: u32 = fn_state.r._0;
        // C s_0_142: const #28s : i
        let s_0_142: i128 = 28;
        // C s_0_143: const #1s : i
        let s_0_143: i128 = 1;
        // D s_0_144: cast zx s_0_141 -> bv
        let s_0_144: Bits = Bits::new(s_0_141 as u128, 32u16);
        // D s_0_145: bit-extract s_0_144 s_0_142 s_0_143
        let s_0_145: Bits = (Bits::new(
            ((s_0_144) >> (s_0_142)).value(),
            u16::try_from(s_0_143).unwrap(),
        ));
        // D s_0_146: cast reint s_0_145 -> u8
        let s_0_146: bool = ((s_0_145.value()) != 0);
        // C s_0_147: const #1s : i
        let s_0_147: i128 = 1;
        // C s_0_148: const #28s : i
        let s_0_148: i128 = 28;
        // D s_0_149: cast zx s_0_140 -> bv
        let s_0_149: Bits = Bits::new(s_0_140 as u128, 32u16);
        // D s_0_150: cast zx s_0_146 -> bv
        let s_0_150: Bits = Bits::new(s_0_146 as u128, 1u16);
        // C s_0_151: const #1u : u64
        let s_0_151: u64 = 1;
        // C s_0_152: cast zx s_0_151 -> bv
        let s_0_152: Bits = Bits::new(s_0_151 as u128, 64u16);
        // C s_0_153: lsl s_0_152 s_0_147
        let s_0_153: Bits = s_0_152 << s_0_147;
        // C s_0_154: sub s_0_153 s_0_152
        let s_0_154: Bits = ((s_0_153) - (s_0_152));
        // D s_0_155: and s_0_150 s_0_154
        let s_0_155: Bits = ((s_0_150) & (s_0_154));
        // D s_0_156: lsl s_0_155 s_0_148
        let s_0_156: Bits = s_0_155 << s_0_148;
        // C s_0_157: lsl s_0_154 s_0_148
        let s_0_157: Bits = s_0_154 << s_0_148;
        // C s_0_158: cmpl s_0_157
        let s_0_158: Bits = !s_0_157;
        // D s_0_159: and s_0_149 s_0_158
        let s_0_159: Bits = ((s_0_149) & (s_0_158));
        // D s_0_160: or s_0_159 s_0_156
        let s_0_160: Bits = ((s_0_159) | (s_0_156));
        // D s_0_161: cast reint s_0_160 -> u32
        let s_0_161: u32 = (s_0_160.value() as u32);
        // D s_0_162: call Mk_DBGDSCRext_Type(s_0_161)
        let s_0_162: ProductType700c18a878c5601b = Mk_DBGDSCRext_Type(
            state,
            tracer,
            s_0_161,
        );
        // D s_0_163: call DBGDSCRext_write(s_0_162)
        let s_0_163: () = DBGDSCRext_write(state, tracer, s_0_162);
        // C s_0_164: const #() : ()
        let s_0_164: () = ();
        // S s_0_165: call DBGOSLSR_read(s_0_164)
        let s_0_165: ProductType700c18a878c5601b = DBGOSLSR_read(state, tracer, s_0_164);
        // S s_0_166: call _get_DBGOSLSR_Type_OSLK(s_0_165)
        let s_0_166: bool = u_get_DBGOSLSR_Type_OSLK(state, tracer, s_0_165);
        // S s_0_167: cast zx s_0_166 -> bv
        let s_0_167: Bits = Bits::new(s_0_166 as u128, 1u16);
        // C s_0_168: const #1u : u8
        let s_0_168: bool = true;
        // C s_0_169: cast zx s_0_168 -> bv
        let s_0_169: Bits = Bits::new(s_0_168 as u128, 1u16);
        // S s_0_170: cmp-eq s_0_167 s_0_169
        let s_0_170: bool = ((s_0_167) == (s_0_169));
        // N s_0_171: branch s_0_170 b29 b1
        if s_0_170 {
            return block_29(state, tracer, fn_state);
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
        // S s_2_1: call DBGOSLSR_read(s_2_0)
        let s_2_1: ProductType700c18a878c5601b = DBGOSLSR_read(state, tracer, s_2_0);
        // S s_2_2: call _get_DBGOSLSR_Type_OSLK(s_2_1)
        let s_2_2: bool = u_get_DBGOSLSR_Type_OSLK(state, tracer, s_2_1);
        // S s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // C s_2_4: const #1u : u8
        let s_2_4: bool = true;
        // C s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 1u16);
        // S s_2_6: cmp-eq s_2_3 s_2_5
        let s_2_6: bool = ((s_2_3) == (s_2_5));
        // N s_2_7: branch s_2_6 b28 b3
        if s_2_6 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call DBGOSLSR_read(s_4_0)
        let s_4_1: ProductType700c18a878c5601b = DBGOSLSR_read(state, tracer, s_4_0);
        // S s_4_2: call _get_DBGOSLSR_Type_OSLK(s_4_1)
        let s_4_2: bool = u_get_DBGOSLSR_Type_OSLK(state, tracer, s_4_1);
        // S s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // C s_4_4: const #1u : u8
        let s_4_4: bool = true;
        // C s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 1u16);
        // S s_4_6: cmp-eq s_4_3 s_4_5
        let s_4_6: bool = ((s_4_3) == (s_4_5));
        // N s_4_7: branch s_4_6 b27 b5
        if s_4_6 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call DBGOSLSR_read(s_6_0)
        let s_6_1: ProductType700c18a878c5601b = DBGOSLSR_read(state, tracer, s_6_0);
        // S s_6_2: call _get_DBGOSLSR_Type_OSLK(s_6_1)
        let s_6_2: bool = u_get_DBGOSLSR_Type_OSLK(state, tracer, s_6_1);
        // S s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // C s_6_4: const #1u : u8
        let s_6_4: bool = true;
        // C s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 1u16);
        // S s_6_6: cmp-eq s_6_3 s_6_5
        let s_6_6: bool = ((s_6_3) == (s_6_5));
        // N s_6_7: branch s_6_6 b26 b7
        if s_6_6 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call DBGOSLSR_read(s_8_0)
        let s_8_1: ProductType700c18a878c5601b = DBGOSLSR_read(state, tracer, s_8_0);
        // S s_8_2: call _get_DBGOSLSR_Type_OSLK(s_8_1)
        let s_8_2: bool = u_get_DBGOSLSR_Type_OSLK(state, tracer, s_8_1);
        // S s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // C s_8_4: const #1u : u8
        let s_8_4: bool = true;
        // C s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 1u16);
        // S s_8_6: cmp-eq s_8_3 s_8_5
        let s_8_6: bool = ((s_8_3) == (s_8_5));
        // N s_8_7: branch s_8_6 b25 b9
        if s_8_6 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call DBGOSLSR_read(s_10_0)
        let s_10_1: ProductType700c18a878c5601b = DBGOSLSR_read(state, tracer, s_10_0);
        // S s_10_2: call _get_DBGOSLSR_Type_OSLK(s_10_1)
        let s_10_2: bool = u_get_DBGOSLSR_Type_OSLK(state, tracer, s_10_1);
        // S s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // C s_10_4: const #1u : u8
        let s_10_4: bool = true;
        // C s_10_5: cast zx s_10_4 -> bv
        let s_10_5: Bits = Bits::new(s_10_4 as u128, 1u16);
        // S s_10_6: cmp-eq s_10_3 s_10_5
        let s_10_6: bool = ((s_10_3) == (s_10_5));
        // N s_10_7: branch s_10_6 b24 b11
        if s_10_6 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call DBGOSLSR_read(s_12_0)
        let s_12_1: ProductType700c18a878c5601b = DBGOSLSR_read(state, tracer, s_12_0);
        // S s_12_2: call _get_DBGOSLSR_Type_OSLK(s_12_1)
        let s_12_2: bool = u_get_DBGOSLSR_Type_OSLK(state, tracer, s_12_1);
        // S s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 1u16);
        // C s_12_4: const #1u : u8
        let s_12_4: bool = true;
        // C s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 1u16);
        // S s_12_6: cmp-eq s_12_3 s_12_5
        let s_12_6: bool = ((s_12_3) == (s_12_5));
        // N s_12_7: branch s_12_6 b23 b13
        if s_12_6 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call DBGOSLSR_read(s_14_0)
        let s_14_1: ProductType700c18a878c5601b = DBGOSLSR_read(state, tracer, s_14_0);
        // S s_14_2: call _get_DBGOSLSR_Type_OSLK(s_14_1)
        let s_14_2: bool = u_get_DBGOSLSR_Type_OSLK(state, tracer, s_14_1);
        // S s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 1u16);
        // C s_14_4: const #1u : u8
        let s_14_4: bool = true;
        // C s_14_5: cast zx s_14_4 -> bv
        let s_14_5: Bits = Bits::new(s_14_4 as u128, 1u16);
        // S s_14_6: cmp-eq s_14_3 s_14_5
        let s_14_6: bool = ((s_14_3) == (s_14_5));
        // N s_14_7: branch s_14_6 b22 b15
        if s_14_6 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call DBGOSLSR_read(s_16_0)
        let s_16_1: ProductType700c18a878c5601b = DBGOSLSR_read(state, tracer, s_16_0);
        // S s_16_2: call _get_DBGOSLSR_Type_OSLK(s_16_1)
        let s_16_2: bool = u_get_DBGOSLSR_Type_OSLK(state, tracer, s_16_1);
        // S s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 1u16);
        // C s_16_4: const #1u : u8
        let s_16_4: bool = true;
        // C s_16_5: cast zx s_16_4 -> bv
        let s_16_5: Bits = Bits::new(s_16_4 as u128, 1u16);
        // S s_16_6: cmp-eq s_16_3 s_16_5
        let s_16_6: bool = ((s_16_3) == (s_16_5));
        // N s_16_7: branch s_16_6 b21 b17
        if s_16_6 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call DBGOSLSR_read(s_18_0)
        let s_18_1: ProductType700c18a878c5601b = DBGOSLSR_read(state, tracer, s_18_0);
        // S s_18_2: call _get_DBGOSLSR_Type_OSLK(s_18_1)
        let s_18_2: bool = u_get_DBGOSLSR_Type_OSLK(state, tracer, s_18_1);
        // S s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // C s_18_4: const #1u : u8
        let s_18_4: bool = true;
        // C s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 1u16);
        // S s_18_6: cmp-eq s_18_3 s_18_5
        let s_18_6: bool = ((s_18_3) == (s_18_5));
        // N s_18_7: branch s_18_6 b20 b19
        if s_18_6 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call DBGDSCRext_read(s_20_0)
        let s_20_1: ProductType700c18a878c5601b = DBGDSCRext_read(state, tracer, s_20_0);
        // D s_20_2: read-var r:struct
        let s_20_2: ProductType700c18a878c5601b = fn_state.r;
        // D s_20_3: call _get_DBGDSCRext_Type_ERR(s_20_2)
        let s_20_3: bool = u_get_DBGDSCRext_Type_ERR(state, tracer, s_20_2);
        // D s_20_4: call _update_DBGDSCRext_Type_ERR(s_20_1, s_20_3)
        let s_20_4: ProductType700c18a878c5601b = u_update_DBGDSCRext_Type_ERR(
            state,
            tracer,
            s_20_1,
            s_20_3,
        );
        // D s_20_5: call DBGDSCRext_write(s_20_4)
        let s_20_5: () = DBGDSCRext_write(state, tracer, s_20_4);
        // N s_20_6: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call DBGDSCRext_read(s_21_0)
        let s_21_1: ProductType700c18a878c5601b = DBGDSCRext_read(state, tracer, s_21_0);
        // D s_21_2: read-var r:struct
        let s_21_2: ProductType700c18a878c5601b = fn_state.r;
        // D s_21_3: call _get_DBGDSCRext_Type_HDE(s_21_2)
        let s_21_3: bool = u_get_DBGDSCRext_Type_HDE(state, tracer, s_21_2);
        // D s_21_4: call _update_DBGDSCRext_Type_HDE(s_21_1, s_21_3)
        let s_21_4: ProductType700c18a878c5601b = u_update_DBGDSCRext_Type_HDE(
            state,
            tracer,
            s_21_1,
            s_21_3,
        );
        // D s_21_5: call DBGDSCRext_write(s_21_4)
        let s_21_5: () = DBGDSCRext_write(state, tracer, s_21_4);
        // N s_21_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call DBGDSCRext_read(s_22_0)
        let s_22_1: ProductType700c18a878c5601b = DBGDSCRext_read(state, tracer, s_22_0);
        // D s_22_2: read-var r:struct
        let s_22_2: ProductType700c18a878c5601b = fn_state.r;
        // D s_22_3: call _get_DBGDSCRext_Type_SC2(s_22_2)
        let s_22_3: bool = u_get_DBGDSCRext_Type_SC2(state, tracer, s_22_2);
        // D s_22_4: call _update_DBGDSCRext_Type_SC2(s_22_1, s_22_3)
        let s_22_4: ProductType700c18a878c5601b = u_update_DBGDSCRext_Type_SC2(
            state,
            tracer,
            s_22_1,
            s_22_3,
        );
        // D s_22_5: call DBGDSCRext_write(s_22_4)
        let s_22_5: () = DBGDSCRext_write(state, tracer, s_22_4);
        // N s_22_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #() : ()
        let s_23_0: () = ();
        // S s_23_1: call DBGDSCRext_read(s_23_0)
        let s_23_1: ProductType700c18a878c5601b = DBGDSCRext_read(state, tracer, s_23_0);
        // D s_23_2: read-var r:struct
        let s_23_2: ProductType700c18a878c5601b = fn_state.r;
        // D s_23_3: call _get_DBGDSCRext_Type_TDA(s_23_2)
        let s_23_3: bool = u_get_DBGDSCRext_Type_TDA(state, tracer, s_23_2);
        // D s_23_4: call _update_DBGDSCRext_Type_TDA(s_23_1, s_23_3)
        let s_23_4: ProductType700c18a878c5601b = u_update_DBGDSCRext_Type_TDA(
            state,
            tracer,
            s_23_1,
            s_23_3,
        );
        // D s_23_5: call DBGDSCRext_write(s_23_4)
        let s_23_5: () = DBGDSCRext_write(state, tracer, s_23_4);
        // N s_23_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call DBGDSCRext_read(s_24_0)
        let s_24_1: ProductType700c18a878c5601b = DBGDSCRext_read(state, tracer, s_24_0);
        // D s_24_2: read-var r:struct
        let s_24_2: ProductType700c18a878c5601b = fn_state.r;
        // D s_24_3: call _get_DBGDSCRext_Type_INTdis(s_24_2)
        let s_24_3: u8 = u_get_DBGDSCRext_Type_INTdis(state, tracer, s_24_2);
        // D s_24_4: call _update_DBGDSCRext_Type_INTdis(s_24_1, s_24_3)
        let s_24_4: ProductType700c18a878c5601b = u_update_DBGDSCRext_Type_INTdis(
            state,
            tracer,
            s_24_1,
            s_24_3,
        );
        // D s_24_5: call DBGDSCRext_write(s_24_4)
        let s_24_5: () = DBGDSCRext_write(state, tracer, s_24_4);
        // N s_24_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call DBGDSCRext_read(s_25_0)
        let s_25_1: ProductType700c18a878c5601b = DBGDSCRext_read(state, tracer, s_25_0);
        // D s_25_2: read-var r:struct
        let s_25_2: ProductType700c18a878c5601b = fn_state.r;
        // D s_25_3: call _get_DBGDSCRext_Type_TXU(s_25_2)
        let s_25_3: bool = u_get_DBGDSCRext_Type_TXU(state, tracer, s_25_2);
        // D s_25_4: call _update_DBGDSCRext_Type_TXU(s_25_1, s_25_3)
        let s_25_4: ProductType700c18a878c5601b = u_update_DBGDSCRext_Type_TXU(
            state,
            tracer,
            s_25_1,
            s_25_3,
        );
        // D s_25_5: call DBGDSCRext_write(s_25_4)
        let s_25_5: () = DBGDSCRext_write(state, tracer, s_25_4);
        // N s_25_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #() : ()
        let s_26_0: () = ();
        // S s_26_1: call DBGDSCRext_read(s_26_0)
        let s_26_1: ProductType700c18a878c5601b = DBGDSCRext_read(state, tracer, s_26_0);
        // D s_26_2: read-var r:struct
        let s_26_2: ProductType700c18a878c5601b = fn_state.r;
        // D s_26_3: call _get_DBGDSCRext_Type_RXO(s_26_2)
        let s_26_3: bool = u_get_DBGDSCRext_Type_RXO(state, tracer, s_26_2);
        // D s_26_4: call _update_DBGDSCRext_Type_RXO(s_26_1, s_26_3)
        let s_26_4: ProductType700c18a878c5601b = u_update_DBGDSCRext_Type_RXO(
            state,
            tracer,
            s_26_1,
            s_26_3,
        );
        // D s_26_5: call DBGDSCRext_write(s_26_4)
        let s_26_5: () = DBGDSCRext_write(state, tracer, s_26_4);
        // N s_26_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call DBGDSCRext_read(s_27_0)
        let s_27_1: ProductType700c18a878c5601b = DBGDSCRext_read(state, tracer, s_27_0);
        // D s_27_2: read-var r:struct
        let s_27_2: ProductType700c18a878c5601b = fn_state.r;
        // D s_27_3: call _get_DBGDSCRext_Type_TXfull(s_27_2)
        let s_27_3: bool = u_get_DBGDSCRext_Type_TXfull(state, tracer, s_27_2);
        // D s_27_4: call _update_DBGDSCRext_Type_TXfull(s_27_1, s_27_3)
        let s_27_4: ProductType700c18a878c5601b = u_update_DBGDSCRext_Type_TXfull(
            state,
            tracer,
            s_27_1,
            s_27_3,
        );
        // D s_27_5: call DBGDSCRext_write(s_27_4)
        let s_27_5: () = DBGDSCRext_write(state, tracer, s_27_4);
        // N s_27_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call DBGDSCRext_read(s_28_0)
        let s_28_1: ProductType700c18a878c5601b = DBGDSCRext_read(state, tracer, s_28_0);
        // D s_28_2: read-var r:struct
        let s_28_2: ProductType700c18a878c5601b = fn_state.r;
        // D s_28_3: call _get_DBGDSCRext_Type_RXfull(s_28_2)
        let s_28_3: bool = u_get_DBGDSCRext_Type_RXfull(state, tracer, s_28_2);
        // D s_28_4: call _update_DBGDSCRext_Type_RXfull(s_28_1, s_28_3)
        let s_28_4: ProductType700c18a878c5601b = u_update_DBGDSCRext_Type_RXfull(
            state,
            tracer,
            s_28_1,
            s_28_3,
        );
        // D s_28_5: call DBGDSCRext_write(s_28_4)
        let s_28_5: () = DBGDSCRext_write(state, tracer, s_28_4);
        // N s_28_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call DBGDSCRext_read(s_29_0)
        let s_29_1: ProductType700c18a878c5601b = DBGDSCRext_read(state, tracer, s_29_0);
        // D s_29_2: read-var r:struct
        let s_29_2: ProductType700c18a878c5601b = fn_state.r;
        // D s_29_3: call _get_DBGDSCRext_Type_TFO(s_29_2)
        let s_29_3: bool = u_get_DBGDSCRext_Type_TFO(state, tracer, s_29_2);
        // D s_29_4: call _update_DBGDSCRext_Type_TFO(s_29_1, s_29_3)
        let s_29_4: ProductType700c18a878c5601b = u_update_DBGDSCRext_Type_TFO(
            state,
            tracer,
            s_29_1,
            s_29_3,
        );
        // D s_29_5: call DBGDSCRext_write(s_29_4)
        let s_29_5: () = DBGDSCRext_write(state, tracer, s_29_4);
        // N s_29_6: jump b2
        return block_2(state, tracer, fn_state);
    }
}
