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
use FFR_read::*;
use u__id::*;
use SP_read::*;
use V_read::*;
use IsFPEnabled::*;
use P_read::*;
use X_read::*;
use HaveGCS::*;
use Z_read::*;
use IsSVEEnabled::*;
use common::*;
pub fn TakeTransactionCheckpoint<T: Tracer>(
    state: &mut State,
    tracer: &T,
    vl: i128,
    pl: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        u_1268: i64,
        ga_19125: u8,
        gs_24514: bool,
        n: i64,
        u_1267: i64,
        vlshadow_468: i128,
        gs_24530: bool,
        gs_24540: bool,
        plshadow_467: i128,
        u_1266: i64,
        vl: i128,
        pl: i128,
    }
    let fn_state = FunctionState {
        vl,
        pl,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var pl:i
        let s_0_0: i128 = fn_state.pl;
        // D s_0_1: write-var plshadow#467 <= s_0_0
        fn_state.plshadow_467 = s_0_0;
        // D s_0_2: read-var vl:i
        let s_0_2: i128 = fn_state.vl;
        // D s_0_3: write-var vlshadow#468 <= s_0_2
        fn_state.vlshadow_468 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call SP_read(s_0_4)
        let s_0_5: u64 = SP_read(state, tracer, s_0_4);
        // C s_0_6: const #91732u : u32
        let s_0_6: u32 = 91732;
        // N s_0_7: write-reg s_0_6 <= s_0_5
        let s_0_7: () = {
            state.write_register::<u64>(s_0_6 as isize, s_0_5);
            tracer.write_register(s_0_6 as isize, s_0_5);
        };
        // C s_0_8: const #12776u : u32
        let s_0_8: u32 = 12776;
        // D s_0_9: read-reg s_0_8:u64
        let s_0_9: u64 = {
            let value = state.read_register::<u64>(s_0_8 as isize);
            tracer.read_register(s_0_8 as isize, value);
            value
        };
        // C s_0_10: const #91196u : u32
        let s_0_10: u32 = 91196;
        // N s_0_11: write-reg s_0_10 <= s_0_9
        let s_0_11: () = {
            state.write_register::<u64>(s_0_10 as isize, s_0_9);
            tracer.write_register(s_0_10 as isize, s_0_9);
        };
        // C s_0_12: const #16984u : u32
        let s_0_12: u32 = 16984;
        // D s_0_13: read-reg s_0_12:u8
        let s_0_13: bool = {
            let value = state.read_register::<bool>(s_0_12 as isize);
            tracer.read_register(s_0_12 as isize, value);
            value
        };
        // C s_0_14: const #16997u : u32
        let s_0_14: u32 = 16997;
        // D s_0_15: read-reg s_0_14:u8
        let s_0_15: bool = {
            let value = state.read_register::<bool>(s_0_14 as isize);
            tracer.read_register(s_0_14 as isize, value);
            value
        };
        // C s_0_16: const #16971u : u32
        let s_0_16: u32 = 16971;
        // D s_0_17: read-reg s_0_16:u8
        let s_0_17: bool = {
            let value = state.read_register::<bool>(s_0_16 as isize);
            tracer.read_register(s_0_16 as isize, value);
            value
        };
        // C s_0_18: const #16996u : u32
        let s_0_18: u32 = 16996;
        // D s_0_19: read-reg s_0_18:u8
        let s_0_19: bool = {
            let value = state.read_register::<bool>(s_0_18 as isize);
            tracer.read_register(s_0_18 as isize, value);
            value
        };
        // D s_0_20: cast zx s_0_17 -> bv
        let s_0_20: Bits = Bits::new(s_0_17 as u128, 1u16);
        // D s_0_21: cast zx s_0_19 -> bv
        let s_0_21: Bits = Bits::new(s_0_19 as u128, 1u16);
        // D s_0_22: cast reint s_0_20 -> u128
        let s_0_22: u128 = (s_0_20.value() as u128);
        // D s_0_23: size-of s_0_20
        let s_0_23: u16 = s_0_20.length();
        // D s_0_24: cast reint s_0_21 -> u128
        let s_0_24: u128 = (s_0_21.value() as u128);
        // D s_0_25: size-of s_0_21
        let s_0_25: u16 = s_0_21.length();
        // D s_0_26: lsl s_0_22 s_0_25
        let s_0_26: u128 = s_0_22 << s_0_25;
        // D s_0_27: or s_0_26 s_0_24
        let s_0_27: u128 = ((s_0_26) | (s_0_24));
        // D s_0_28: add s_0_23 s_0_25
        let s_0_28: u16 = (s_0_23 + s_0_25);
        // D s_0_29: create-bits s_0_27 s_0_28
        let s_0_29: Bits = Bits::new(s_0_27, s_0_28);
        // D s_0_30: cast reint s_0_29 -> u8
        let s_0_30: u8 = (s_0_29.value() as u8);
        // D s_0_31: cast zx s_0_15 -> bv
        let s_0_31: Bits = Bits::new(s_0_15 as u128, 1u16);
        // D s_0_32: cast zx s_0_30 -> bv
        let s_0_32: Bits = Bits::new(s_0_30 as u128, 2u16);
        // D s_0_33: cast reint s_0_31 -> u128
        let s_0_33: u128 = (s_0_31.value() as u128);
        // D s_0_34: size-of s_0_31
        let s_0_34: u16 = s_0_31.length();
        // D s_0_35: cast reint s_0_32 -> u128
        let s_0_35: u128 = (s_0_32.value() as u128);
        // D s_0_36: size-of s_0_32
        let s_0_36: u16 = s_0_32.length();
        // D s_0_37: lsl s_0_33 s_0_36
        let s_0_37: u128 = s_0_33 << s_0_36;
        // D s_0_38: or s_0_37 s_0_35
        let s_0_38: u128 = ((s_0_37) | (s_0_35));
        // D s_0_39: add s_0_34 s_0_36
        let s_0_39: u16 = (s_0_34 + s_0_36);
        // D s_0_40: create-bits s_0_38 s_0_39
        let s_0_40: Bits = Bits::new(s_0_38, s_0_39);
        // D s_0_41: cast reint s_0_40 -> u8
        let s_0_41: u8 = (s_0_40.value() as u8);
        // D s_0_42: cast zx s_0_13 -> bv
        let s_0_42: Bits = Bits::new(s_0_13 as u128, 1u16);
        // D s_0_43: cast zx s_0_41 -> bv
        let s_0_43: Bits = Bits::new(s_0_41 as u128, 3u16);
        // D s_0_44: cast reint s_0_42 -> u128
        let s_0_44: u128 = (s_0_42.value() as u128);
        // D s_0_45: size-of s_0_42
        let s_0_45: u16 = s_0_42.length();
        // D s_0_46: cast reint s_0_43 -> u128
        let s_0_46: u128 = (s_0_43.value() as u128);
        // D s_0_47: size-of s_0_43
        let s_0_47: u16 = s_0_43.length();
        // D s_0_48: lsl s_0_44 s_0_47
        let s_0_48: u128 = s_0_44 << s_0_47;
        // D s_0_49: or s_0_48 s_0_46
        let s_0_49: u128 = ((s_0_48) | (s_0_46));
        // D s_0_50: add s_0_45 s_0_47
        let s_0_50: u16 = (s_0_45 + s_0_47);
        // D s_0_51: create-bits s_0_49 s_0_50
        let s_0_51: Bits = Bits::new(s_0_49, s_0_50);
        // D s_0_52: cast reint s_0_51 -> u8
        let s_0_52: u8 = (s_0_51.value() as u8);
        // C s_0_53: const #100204u : u32
        let s_0_53: u32 = 100204;
        // N s_0_54: write-reg s_0_53 <= s_0_52
        let s_0_54: () = {
            state.write_register::<u8>(s_0_53 as isize, s_0_52);
            tracer.write_register(s_0_53 as isize, s_0_52);
        };
        // C s_0_55: const #16972u : u32
        let s_0_55: u32 = 16972;
        // D s_0_56: read-reg s_0_55:u8
        let s_0_56: bool = {
            let value = state.read_register::<bool>(s_0_55 as isize);
            tracer.read_register(s_0_55 as isize, value);
            value
        };
        // C s_0_57: const #16968u : u32
        let s_0_57: u32 = 16968;
        // D s_0_58: read-reg s_0_57:u8
        let s_0_58: bool = {
            let value = state.read_register::<bool>(s_0_57 as isize);
            tracer.read_register(s_0_57 as isize, value);
            value
        };
        // C s_0_59: const #16979u : u32
        let s_0_59: u32 = 16979;
        // D s_0_60: read-reg s_0_59:u8
        let s_0_60: bool = {
            let value = state.read_register::<bool>(s_0_59 as isize);
            tracer.read_register(s_0_59 as isize, value);
            value
        };
        // C s_0_61: const #16977u : u32
        let s_0_61: u32 = 16977;
        // D s_0_62: read-reg s_0_61:u8
        let s_0_62: bool = {
            let value = state.read_register::<bool>(s_0_61 as isize);
            tracer.read_register(s_0_61 as isize, value);
            value
        };
        // D s_0_63: cast zx s_0_60 -> bv
        let s_0_63: Bits = Bits::new(s_0_60 as u128, 1u16);
        // D s_0_64: cast zx s_0_62 -> bv
        let s_0_64: Bits = Bits::new(s_0_62 as u128, 1u16);
        // D s_0_65: cast reint s_0_63 -> u128
        let s_0_65: u128 = (s_0_63.value() as u128);
        // D s_0_66: size-of s_0_63
        let s_0_66: u16 = s_0_63.length();
        // D s_0_67: cast reint s_0_64 -> u128
        let s_0_67: u128 = (s_0_64.value() as u128);
        // D s_0_68: size-of s_0_64
        let s_0_68: u16 = s_0_64.length();
        // D s_0_69: lsl s_0_65 s_0_68
        let s_0_69: u128 = s_0_65 << s_0_68;
        // D s_0_70: or s_0_69 s_0_67
        let s_0_70: u128 = ((s_0_69) | (s_0_67));
        // D s_0_71: add s_0_66 s_0_68
        let s_0_71: u16 = (s_0_66 + s_0_68);
        // D s_0_72: create-bits s_0_70 s_0_71
        let s_0_72: Bits = Bits::new(s_0_70, s_0_71);
        // D s_0_73: cast reint s_0_72 -> u8
        let s_0_73: u8 = (s_0_72.value() as u8);
        // D s_0_74: cast zx s_0_58 -> bv
        let s_0_74: Bits = Bits::new(s_0_58 as u128, 1u16);
        // D s_0_75: cast zx s_0_73 -> bv
        let s_0_75: Bits = Bits::new(s_0_73 as u128, 2u16);
        // D s_0_76: cast reint s_0_74 -> u128
        let s_0_76: u128 = (s_0_74.value() as u128);
        // D s_0_77: size-of s_0_74
        let s_0_77: u16 = s_0_74.length();
        // D s_0_78: cast reint s_0_75 -> u128
        let s_0_78: u128 = (s_0_75.value() as u128);
        // D s_0_79: size-of s_0_75
        let s_0_79: u16 = s_0_75.length();
        // D s_0_80: lsl s_0_76 s_0_79
        let s_0_80: u128 = s_0_76 << s_0_79;
        // D s_0_81: or s_0_80 s_0_78
        let s_0_81: u128 = ((s_0_80) | (s_0_78));
        // D s_0_82: add s_0_77 s_0_79
        let s_0_82: u16 = (s_0_77 + s_0_79);
        // D s_0_83: create-bits s_0_81 s_0_82
        let s_0_83: Bits = Bits::new(s_0_81, s_0_82);
        // D s_0_84: cast reint s_0_83 -> u8
        let s_0_84: u8 = (s_0_83.value() as u8);
        // D s_0_85: cast zx s_0_56 -> bv
        let s_0_85: Bits = Bits::new(s_0_56 as u128, 1u16);
        // D s_0_86: cast zx s_0_84 -> bv
        let s_0_86: Bits = Bits::new(s_0_84 as u128, 3u16);
        // D s_0_87: cast reint s_0_85 -> u128
        let s_0_87: u128 = (s_0_85.value() as u128);
        // D s_0_88: size-of s_0_85
        let s_0_88: u16 = s_0_85.length();
        // D s_0_89: cast reint s_0_86 -> u128
        let s_0_89: u128 = (s_0_86.value() as u128);
        // D s_0_90: size-of s_0_86
        let s_0_90: u16 = s_0_86.length();
        // D s_0_91: lsl s_0_87 s_0_90
        let s_0_91: u128 = s_0_87 << s_0_90;
        // D s_0_92: or s_0_91 s_0_89
        let s_0_92: u128 = ((s_0_91) | (s_0_89));
        // D s_0_93: add s_0_88 s_0_90
        let s_0_93: u16 = (s_0_88 + s_0_90);
        // D s_0_94: create-bits s_0_92 s_0_93
        let s_0_94: Bits = Bits::new(s_0_92, s_0_93);
        // D s_0_95: cast reint s_0_94 -> u8
        let s_0_95: u8 = (s_0_94.value() as u8);
        // C s_0_96: const #3s : i
        let s_0_96: i128 = 3;
        // D s_0_97: cast zx s_0_95 -> bv
        let s_0_97: Bits = Bits::new(s_0_95 as u128, 4u16);
        // C s_0_98: const #1s : i64
        let s_0_98: i64 = 1;
        // C s_0_99: cast zx s_0_98 -> i
        let s_0_99: i128 = (i128::try_from(s_0_98).unwrap());
        // C s_0_100: const #0s : i
        let s_0_100: i128 = 0;
        // C s_0_101: add s_0_100 s_0_99
        let s_0_101: i128 = (s_0_100 + s_0_99);
        // D s_0_102: bit-extract s_0_97 s_0_96 s_0_101
        let s_0_102: Bits = (Bits::new(
            ((s_0_97) >> (s_0_96)).value(),
            u16::try_from(s_0_101).unwrap(),
        ));
        // D s_0_103: cast reint s_0_102 -> u8
        let s_0_103: bool = ((s_0_102.value()) != 0);
        // C s_0_104: const #91137u : u32
        let s_0_104: u32 = 91137;
        // N s_0_105: write-reg s_0_104 <= s_0_103
        let s_0_105: () = {
            state.write_register::<bool>(s_0_104 as isize, s_0_103);
            tracer.write_register(s_0_104 as isize, s_0_103);
        };
        // C s_0_106: const #2s : i
        let s_0_106: i128 = 2;
        // D s_0_107: cast zx s_0_95 -> bv
        let s_0_107: Bits = Bits::new(s_0_95 as u128, 4u16);
        // C s_0_108: const #1s : i64
        let s_0_108: i64 = 1;
        // C s_0_109: cast zx s_0_108 -> i
        let s_0_109: i128 = (i128::try_from(s_0_108).unwrap());
        // C s_0_110: const #0s : i
        let s_0_110: i128 = 0;
        // C s_0_111: add s_0_110 s_0_109
        let s_0_111: i128 = (s_0_110 + s_0_109);
        // D s_0_112: bit-extract s_0_107 s_0_106 s_0_111
        let s_0_112: Bits = (Bits::new(
            ((s_0_107) >> (s_0_106)).value(),
            u16::try_from(s_0_111).unwrap(),
        ));
        // D s_0_113: cast reint s_0_112 -> u8
        let s_0_113: bool = ((s_0_112.value()) != 0);
        // C s_0_114: const #91136u : u32
        let s_0_114: u32 = 91136;
        // N s_0_115: write-reg s_0_114 <= s_0_113
        let s_0_115: () = {
            state.write_register::<bool>(s_0_114 as isize, s_0_113);
            tracer.write_register(s_0_114 as isize, s_0_113);
        };
        // C s_0_116: const #1s : i
        let s_0_116: i128 = 1;
        // D s_0_117: cast zx s_0_95 -> bv
        let s_0_117: Bits = Bits::new(s_0_95 as u128, 4u16);
        // C s_0_118: const #1s : i64
        let s_0_118: i64 = 1;
        // C s_0_119: cast zx s_0_118 -> i
        let s_0_119: i128 = (i128::try_from(s_0_118).unwrap());
        // C s_0_120: const #0s : i
        let s_0_120: i128 = 0;
        // C s_0_121: add s_0_120 s_0_119
        let s_0_121: i128 = (s_0_120 + s_0_119);
        // D s_0_122: bit-extract s_0_117 s_0_116 s_0_121
        let s_0_122: Bits = (Bits::new(
            ((s_0_117) >> (s_0_116)).value(),
            u16::try_from(s_0_121).unwrap(),
        ));
        // D s_0_123: cast reint s_0_122 -> u8
        let s_0_123: bool = ((s_0_122.value()) != 0);
        // C s_0_124: const #91195u : u32
        let s_0_124: u32 = 91195;
        // N s_0_125: write-reg s_0_124 <= s_0_123
        let s_0_125: () = {
            state.write_register::<bool>(s_0_124 as isize, s_0_123);
            tracer.write_register(s_0_124 as isize, s_0_123);
        };
        // C s_0_126: const #0s : i
        let s_0_126: i128 = 0;
        // D s_0_127: cast zx s_0_95 -> bv
        let s_0_127: Bits = Bits::new(s_0_95 as u128, 4u16);
        // C s_0_128: const #1s : i64
        let s_0_128: i64 = 1;
        // C s_0_129: cast zx s_0_128 -> i
        let s_0_129: i128 = (i128::try_from(s_0_128).unwrap());
        // C s_0_130: const #0s : i
        let s_0_130: i128 = 0;
        // C s_0_131: add s_0_130 s_0_129
        let s_0_131: i128 = (s_0_130 + s_0_129);
        // D s_0_132: bit-extract s_0_127 s_0_126 s_0_131
        let s_0_132: Bits = (Bits::new(
            ((s_0_127) >> (s_0_126)).value(),
            u16::try_from(s_0_131).unwrap(),
        ));
        // D s_0_133: cast reint s_0_132 -> u8
        let s_0_133: bool = ((s_0_132.value()) != 0);
        // C s_0_134: const #91138u : u32
        let s_0_134: u32 = 91138;
        // N s_0_135: write-reg s_0_134 <= s_0_133
        let s_0_135: () = {
            state.write_register::<bool>(s_0_134 as isize, s_0_133);
            tracer.write_register(s_0_134 as isize, s_0_133);
        };
        // C s_0_136: const #0s : i64
        let s_0_136: i64 = 0;
        // D s_0_137: write-var n <= s_0_136
        fn_state.n = s_0_136;
        // N s_0_138: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var n:i64
        let s_1_0: i64 = fn_state.n;
        // C s_1_1: const #30s : i64
        let s_1_1: i64 = 30;
        // D s_1_2: cmp-gt s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) > (s_1_1));
        // N s_1_3: branch s_1_2 b3 b2
        if s_1_2 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #64s : i64
        let s_2_0: i64 = 64;
        // D s_2_1: read-var n:i64
        let s_2_1: i64 = fn_state.n;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: call X_read(s_2_2, s_2_0)
        let s_2_3: Bits = X_read(state, tracer, s_2_2, s_2_0);
        // C s_2_4: const #91136u : u32
        let s_2_4: u32 = 91136;
        // D s_2_5: read-reg s_2_4:struct
        let s_2_5: ProductType887a02170738ab3b = {
            let value = state
                .read_register::<ProductType887a02170738ab3b>(s_2_4 as isize);
            tracer.read_register(s_2_4 as isize, value);
            value
        };
        // C s_2_6: const #91136u : u32
        let s_2_6: u32 = 91136;
        // N s_2_7: write-reg s_2_6 <= s_2_5
        let s_2_7: () = {
            state.write_register::<ProductType887a02170738ab3b>(s_2_6 as isize, s_2_5);
            tracer.write_register(s_2_6 as isize, s_2_5);
        };
        // D s_2_8: read-var n:i64
        let s_2_8: i64 = fn_state.n;
        // C s_2_9: const #1s : i64
        let s_2_9: i64 = 1;
        // D s_2_10: add s_2_8 s_2_9
        let s_2_10: i64 = (s_2_8 + s_2_9);
        // D s_2_11: write-var n <= s_2_10
        fn_state.n = s_2_10;
        // N s_2_12: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #16975u : u32
        let s_3_0: u32 = 16975;
        // D s_3_1: read-reg s_3_0:u8
        let s_3_1: u8 = {
            let value = state.read_register::<u8>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: call IsFPEnabled(s_3_1)
        let s_3_2: bool = IsFPEnabled(state, tracer, s_3_1);
        // N s_3_3: branch s_3_2 b16 b4
        if s_3_2 {
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
        // N s_4_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call HaveGCS(s_5_0)
        let s_5_1: bool = HaveGCS(state, tracer, s_5_0);
        // N s_5_2: branch s_5_1 b7 b6
        if s_5_1 {
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
        // N s_6_0: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #16975u : u32
        let s_7_0: u32 = 16975;
        // D s_7_1: read-reg s_7_0:u8
        let s_7_1: u8 = {
            let value = state.read_register::<u8>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: write-var ga#19125 <= s_7_1
        fn_state.ga_19125 = s_7_1;
        // D s_7_3: read-var ga#19125:u8
        let s_7_3: u8 = fn_state.ga_19125;
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 2u16);
        // C s_7_5: const #448u : u32
        let s_7_5: u32 = 448;
        // D s_7_6: read-reg s_7_5:u8
        let s_7_6: u8 = {
            let value = state.read_register::<u8>(s_7_5 as isize);
            tracer.read_register(s_7_5 as isize, value);
            value
        };
        // D s_7_7: cast zx s_7_6 -> bv
        let s_7_7: Bits = Bits::new(s_7_6 as u128, 2u16);
        // D s_7_8: cmp-eq s_7_4 s_7_7
        let s_7_8: bool = ((s_7_4) == (s_7_7));
        // D s_7_9: not s_7_8
        let s_7_9: bool = !s_7_8;
        // N s_7_10: branch s_7_9 b9 b8
        if s_7_9 {
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
        // C s_8_0: const #20840u : u32
        let s_8_0: u32 = 20840;
        // D s_8_1: read-reg s_8_0:u64
        let s_8_1: u64 = {
            let value = state.read_register::<u64>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // C s_8_2: const #91187u : u32
        let s_8_2: u32 = 91187;
        // N s_8_3: write-reg s_8_2 <= s_8_1
        let s_8_3: () = {
            state.write_register::<u64>(s_8_2 as isize, s_8_1);
            tracer.write_register(s_8_2 as isize, s_8_1);
        };
        // N s_8_4: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var ga#19125:u8
        let s_9_0: u8 = fn_state.ga_19125;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 2u16);
        // C s_9_2: const #440u : u32
        let s_9_2: u32 = 440;
        // D s_9_3: read-reg s_9_2:u8
        let s_9_3: u8 = {
            let value = state.read_register::<u8>(s_9_2 as isize);
            tracer.read_register(s_9_2 as isize, value);
            value
        };
        // D s_9_4: cast zx s_9_3 -> bv
        let s_9_4: Bits = Bits::new(s_9_3 as u128, 2u16);
        // D s_9_5: cmp-eq s_9_1 s_9_4
        let s_9_5: bool = ((s_9_1) == (s_9_4));
        // D s_9_6: not s_9_5
        let s_9_6: bool = !s_9_5;
        // N s_9_7: branch s_9_6 b11 b10
        if s_9_6 {
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
        // C s_10_0: const #21912u : u32
        let s_10_0: u32 = 21912;
        // D s_10_1: read-reg s_10_0:u64
        let s_10_1: u64 = {
            let value = state.read_register::<u64>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // C s_10_2: const #91187u : u32
        let s_10_2: u32 = 91187;
        // N s_10_3: write-reg s_10_2 <= s_10_1
        let s_10_3: () = {
            state.write_register::<u64>(s_10_2 as isize, s_10_1);
            tracer.write_register(s_10_2 as isize, s_10_1);
        };
        // N s_10_4: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var ga#19125:u8
        let s_11_0: u8 = fn_state.ga_19125;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 2u16);
        // C s_11_2: const #432u : u32
        let s_11_2: u32 = 432;
        // D s_11_3: read-reg s_11_2:u8
        let s_11_3: u8 = {
            let value = state.read_register::<u8>(s_11_2 as isize);
            tracer.read_register(s_11_2 as isize, value);
            value
        };
        // D s_11_4: cast zx s_11_3 -> bv
        let s_11_4: Bits = Bits::new(s_11_3 as u128, 2u16);
        // D s_11_5: cmp-eq s_11_1 s_11_4
        let s_11_5: bool = ((s_11_1) == (s_11_4));
        // D s_11_6: not s_11_5
        let s_11_6: bool = !s_11_5;
        // N s_11_7: branch s_11_6 b13 b12
        if s_11_6 {
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
        // C s_12_0: const #10216u : u32
        let s_12_0: u32 = 10216;
        // D s_12_1: read-reg s_12_0:u64
        let s_12_1: u64 = {
            let value = state.read_register::<u64>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // C s_12_2: const #91187u : u32
        let s_12_2: u32 = 91187;
        // N s_12_3: write-reg s_12_2 <= s_12_1
        let s_12_3: () = {
            state.write_register::<u64>(s_12_2 as isize, s_12_1);
            tracer.write_register(s_12_2 as isize, s_12_1);
        };
        // N s_12_4: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var ga#19125:u8
        let s_13_0: u8 = fn_state.ga_19125;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 2u16);
        // C s_13_2: const #424u : u32
        let s_13_2: u32 = 424;
        // D s_13_3: read-reg s_13_2:u8
        let s_13_3: u8 = {
            let value = state.read_register::<u8>(s_13_2 as isize);
            tracer.read_register(s_13_2 as isize, value);
            value
        };
        // D s_13_4: cast zx s_13_3 -> bv
        let s_13_4: Bits = Bits::new(s_13_3 as u128, 2u16);
        // D s_13_5: cmp-eq s_13_1 s_13_4
        let s_13_5: bool = ((s_13_1) == (s_13_4));
        // D s_13_6: not s_13_5
        let s_13_6: bool = !s_13_5;
        // N s_13_7: branch s_13_6 b15 b14
        if s_13_6 {
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
        // C s_14_0: const #18248u : u32
        let s_14_0: u32 = 18248;
        // D s_14_1: read-reg s_14_0:u64
        let s_14_1: u64 = {
            let value = state.read_register::<u64>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // C s_14_2: const #91187u : u32
        let s_14_2: u32 = 91187;
        // N s_14_3: write-reg s_14_2 <= s_14_1
        let s_14_3: () = {
            state.write_register::<u64>(s_14_2 as isize, s_14_1);
            tracer.write_register(s_14_2 as isize, s_14_1);
        };
        // N s_14_4: return
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
        // C s_16_0: const #16975u : u32
        let s_16_0: u32 = 16975;
        // D s_16_1: read-reg s_16_0:u8
        let s_16_1: u8 = {
            let value = state.read_register::<u8>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: call IsSVEEnabled(s_16_1)
        let s_16_2: bool = IsSVEEnabled(state, tracer, s_16_1);
        // N s_16_3: branch s_16_2 b22 b17
        if s_16_2 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0s : i64
        let s_17_0: i64 = 0;
        // D s_17_1: write-var u#1268 <= s_17_0
        fn_state.u_1268 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var u#1268:i64
        let s_18_0: i64 = fn_state.u_1268;
        // C s_18_1: const #31s : i64
        let s_18_1: i64 = 31;
        // D s_18_2: cmp-gt s_18_0 s_18_1
        let s_18_2: bool = ((s_18_0) > (s_18_1));
        // N s_18_3: branch s_18_2 b20 b19
        if s_18_2 {
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
        // C s_19_0: const #128s : i64
        let s_19_0: i64 = 128;
        // D s_19_1: read-var u#1268:i64
        let s_19_1: i64 = fn_state.u_1268;
        // D s_19_2: cast zx s_19_1 -> i
        let s_19_2: i128 = (i128::try_from(s_19_1).unwrap());
        // D s_19_3: call V_read(s_19_2, s_19_0)
        let s_19_3: Bits = V_read(state, tracer, s_19_2, s_19_0);
        // C s_19_4: const #91136u : u32
        let s_19_4: u32 = 91136;
        // D s_19_5: read-reg s_19_4:struct
        let s_19_5: ProductType887a02170738ab3b = {
            let value = state
                .read_register::<ProductType887a02170738ab3b>(s_19_4 as isize);
            tracer.read_register(s_19_4 as isize, value);
            value
        };
        // C s_19_6: const #91136u : u32
        let s_19_6: u32 = 91136;
        // N s_19_7: write-reg s_19_6 <= s_19_5
        let s_19_7: () = {
            state.write_register::<ProductType887a02170738ab3b>(s_19_6 as isize, s_19_5);
            tracer.write_register(s_19_6 as isize, s_19_5);
        };
        // D s_19_8: read-var u#1268:i64
        let s_19_8: i64 = fn_state.u_1268;
        // C s_19_9: const #1s : i64
        let s_19_9: i64 = 1;
        // D s_19_10: add s_19_8 s_19_9
        let s_19_10: i64 = (s_19_8 + s_19_9);
        // D s_19_11: write-var u#1268 <= s_19_10
        fn_state.u_1268 = s_19_10;
        // N s_19_12: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #12920u : u32
        let s_21_0: u32 = 12920;
        // D s_21_1: read-reg s_21_0:u64
        let s_21_1: u64 = {
            let value = state.read_register::<u64>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // C s_21_2: const #91171u : u32
        let s_21_2: u32 = 91171;
        // N s_21_3: write-reg s_21_2 <= s_21_1
        let s_21_3: () = {
            state.write_register::<u64>(s_21_2 as isize, s_21_1);
            tracer.write_register(s_21_2 as isize, s_21_1);
        };
        // C s_21_4: const #20696u : u32
        let s_21_4: u32 = 20696;
        // D s_21_5: read-reg s_21_4:u64
        let s_21_5: u64 = {
            let value = state.read_register::<u64>(s_21_4 as isize);
            tracer.read_register(s_21_4 as isize, value);
            value
        };
        // C s_21_6: const #91179u : u32
        let s_21_6: u32 = 91179;
        // N s_21_7: write-reg s_21_6 <= s_21_5
        let s_21_7: () = {
            state.write_register::<u64>(s_21_6 as isize, s_21_5);
            tracer.write_register(s_21_6 as isize, s_21_5);
        };
        // N s_21_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #0s : i64
        let s_22_0: i64 = 0;
        // D s_22_1: write-var u#1266 <= s_22_0
        fn_state.u_1266 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var u#1266:i64
        let s_23_0: i64 = fn_state.u_1266;
        // C s_23_1: const #31s : i64
        let s_23_1: i64 = 31;
        // D s_23_2: cmp-gt s_23_0 s_23_1
        let s_23_2: bool = ((s_23_0) > (s_23_1));
        // N s_23_3: branch s_23_2 b28 b24
        if s_23_2 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var vlshadow#468:i
        let s_24_0: i128 = fn_state.vlshadow_468;
        // D s_24_1: call __id(s_24_0)
        let s_24_1: i128 = u__id(state, tracer, s_24_0);
        // C s_24_2: const #1s : i
        let s_24_2: i128 = 1;
        // D s_24_3: sub s_24_1 s_24_2
        let s_24_3: i128 = ((s_24_1) - (s_24_2));
        // C s_24_4: const #0s : i
        let s_24_4: i128 = 0;
        // D s_24_5: cmp-le s_24_4 s_24_3
        let s_24_5: bool = ((s_24_4) <= (s_24_3));
        // N s_24_6: branch s_24_5 b27 b25
        if s_24_5 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#24514 <= s_25_0
        fn_state.gs_24514 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#24514:u8
        let s_26_0: bool = fn_state.gs_24514;
        // N s_26_1: assert s_26_0
        let s_26_1: () = assert!(s_26_0);
        // D s_26_2: read-var vlshadow#468:i
        let s_26_2: i128 = fn_state.vlshadow_468;
        // D s_26_3: cast reint s_26_2 -> i64
        let s_26_3: i64 = (s_26_2 as i64);
        // D s_26_4: read-var u#1266:i64
        let s_26_4: i64 = fn_state.u_1266;
        // D s_26_5: cast zx s_26_4 -> i
        let s_26_5: i128 = (i128::try_from(s_26_4).unwrap());
        // D s_26_6: cast zx s_26_3 -> i
        let s_26_6: i128 = (i128::try_from(s_26_3).unwrap());
        // D s_26_7: call Z_read(s_26_5, s_26_6)
        let s_26_7: Bits = Z_read(state, tracer, s_26_5, s_26_6);
        // C s_26_8: const #91136u : u32
        let s_26_8: u32 = 91136;
        // D s_26_9: read-reg s_26_8:struct
        let s_26_9: ProductType887a02170738ab3b = {
            let value = state
                .read_register::<ProductType887a02170738ab3b>(s_26_8 as isize);
            tracer.read_register(s_26_8 as isize, value);
            value
        };
        // C s_26_10: const #91136u : u32
        let s_26_10: u32 = 91136;
        // N s_26_11: write-reg s_26_10 <= s_26_9
        let s_26_11: () = {
            state
                .write_register::<ProductType887a02170738ab3b>(s_26_10 as isize, s_26_9);
            tracer.write_register(s_26_10 as isize, s_26_9);
        };
        // D s_26_12: read-var u#1266:i64
        let s_26_12: i64 = fn_state.u_1266;
        // C s_26_13: const #1s : i64
        let s_26_13: i64 = 1;
        // D s_26_14: add s_26_12 s_26_13
        let s_26_14: i64 = (s_26_12 + s_26_13);
        // D s_26_15: write-var u#1266 <= s_26_14
        fn_state.u_1266 = s_26_14;
        // N s_26_16: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var vlshadow#468:i
        let s_27_0: i128 = fn_state.vlshadow_468;
        // D s_27_1: call __id(s_27_0)
        let s_27_1: i128 = u__id(state, tracer, s_27_0);
        // C s_27_2: const #1s : i
        let s_27_2: i128 = 1;
        // D s_27_3: sub s_27_1 s_27_2
        let s_27_3: i128 = ((s_27_1) - (s_27_2));
        // C s_27_4: const #2048s : i
        let s_27_4: i128 = 2048;
        // D s_27_5: cmp-lt s_27_3 s_27_4
        let s_27_5: bool = ((s_27_3) < (s_27_4));
        // D s_27_6: write-var gs#24514 <= s_27_5
        fn_state.gs_24514 = s_27_5;
        // N s_27_7: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #0s : i64
        let s_28_0: i64 = 0;
        // D s_28_1: write-var u#1267 <= s_28_0
        fn_state.u_1267 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var u#1267:i64
        let s_29_0: i64 = fn_state.u_1267;
        // C s_29_1: const #15s : i64
        let s_29_1: i64 = 15;
        // D s_29_2: cmp-gt s_29_0 s_29_1
        let s_29_2: bool = ((s_29_0) > (s_29_1));
        // N s_29_3: branch s_29_2 b34 b30
        if s_29_2 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var plshadow#467:i
        let s_30_0: i128 = fn_state.plshadow_467;
        // D s_30_1: call __id(s_30_0)
        let s_30_1: i128 = u__id(state, tracer, s_30_0);
        // C s_30_2: const #1s : i
        let s_30_2: i128 = 1;
        // D s_30_3: sub s_30_1 s_30_2
        let s_30_3: i128 = ((s_30_1) - (s_30_2));
        // C s_30_4: const #0s : i
        let s_30_4: i128 = 0;
        // D s_30_5: cmp-le s_30_4 s_30_3
        let s_30_5: bool = ((s_30_4) <= (s_30_3));
        // N s_30_6: branch s_30_5 b33 b31
        if s_30_5 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #0u : u8
        let s_31_0: bool = false;
        // D s_31_1: write-var gs#24530 <= s_31_0
        fn_state.gs_24530 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#24530:u8
        let s_32_0: bool = fn_state.gs_24530;
        // N s_32_1: assert s_32_0
        let s_32_1: () = assert!(s_32_0);
        // D s_32_2: read-var plshadow#467:i
        let s_32_2: i128 = fn_state.plshadow_467;
        // D s_32_3: cast reint s_32_2 -> i64
        let s_32_3: i64 = (s_32_2 as i64);
        // D s_32_4: read-var u#1267:i64
        let s_32_4: i64 = fn_state.u_1267;
        // D s_32_5: cast zx s_32_4 -> i
        let s_32_5: i128 = (i128::try_from(s_32_4).unwrap());
        // D s_32_6: cast zx s_32_3 -> i
        let s_32_6: i128 = (i128::try_from(s_32_3).unwrap());
        // D s_32_7: call P_read(s_32_5, s_32_6)
        let s_32_7: Bits = P_read(state, tracer, s_32_5, s_32_6);
        // C s_32_8: const #91136u : u32
        let s_32_8: u32 = 91136;
        // D s_32_9: read-reg s_32_8:struct
        let s_32_9: ProductType887a02170738ab3b = {
            let value = state
                .read_register::<ProductType887a02170738ab3b>(s_32_8 as isize);
            tracer.read_register(s_32_8 as isize, value);
            value
        };
        // C s_32_10: const #91136u : u32
        let s_32_10: u32 = 91136;
        // N s_32_11: write-reg s_32_10 <= s_32_9
        let s_32_11: () = {
            state
                .write_register::<ProductType887a02170738ab3b>(s_32_10 as isize, s_32_9);
            tracer.write_register(s_32_10 as isize, s_32_9);
        };
        // D s_32_12: read-var u#1267:i64
        let s_32_12: i64 = fn_state.u_1267;
        // C s_32_13: const #1s : i64
        let s_32_13: i64 = 1;
        // D s_32_14: add s_32_12 s_32_13
        let s_32_14: i64 = (s_32_12 + s_32_13);
        // D s_32_15: write-var u#1267 <= s_32_14
        fn_state.u_1267 = s_32_14;
        // N s_32_16: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var plshadow#467:i
        let s_33_0: i128 = fn_state.plshadow_467;
        // D s_33_1: call __id(s_33_0)
        let s_33_1: i128 = u__id(state, tracer, s_33_0);
        // C s_33_2: const #1s : i
        let s_33_2: i128 = 1;
        // D s_33_3: sub s_33_1 s_33_2
        let s_33_3: i128 = ((s_33_1) - (s_33_2));
        // C s_33_4: const #256s : i
        let s_33_4: i128 = 256;
        // D s_33_5: cmp-lt s_33_3 s_33_4
        let s_33_5: bool = ((s_33_3) < (s_33_4));
        // D s_33_6: write-var gs#24530 <= s_33_5
        fn_state.gs_24530 = s_33_5;
        // N s_33_7: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var plshadow#467:i
        let s_34_0: i128 = fn_state.plshadow_467;
        // D s_34_1: call __id(s_34_0)
        let s_34_1: i128 = u__id(state, tracer, s_34_0);
        // C s_34_2: const #1s : i
        let s_34_2: i128 = 1;
        // D s_34_3: sub s_34_1 s_34_2
        let s_34_3: i128 = ((s_34_1) - (s_34_2));
        // C s_34_4: const #0s : i
        let s_34_4: i128 = 0;
        // D s_34_5: cmp-le s_34_4 s_34_3
        let s_34_5: bool = ((s_34_4) <= (s_34_3));
        // N s_34_6: branch s_34_5 b37 b35
        if s_34_5 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // D s_35_1: write-var gs#24540 <= s_35_0
        fn_state.gs_24540 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#24540:u8
        let s_36_0: bool = fn_state.gs_24540;
        // N s_36_1: assert s_36_0
        let s_36_1: () = assert!(s_36_0);
        // D s_36_2: read-var plshadow#467:i
        let s_36_2: i128 = fn_state.plshadow_467;
        // D s_36_3: cast reint s_36_2 -> i64
        let s_36_3: i64 = (s_36_2 as i64);
        // D s_36_4: cast zx s_36_3 -> i
        let s_36_4: i128 = (i128::try_from(s_36_3).unwrap());
        // D s_36_5: call FFR_read(s_36_4)
        let s_36_5: Bits = FFR_read(state, tracer, s_36_4);
        // C s_36_6: const #91136u : u32
        let s_36_6: u32 = 91136;
        // D s_36_7: read-reg s_36_6:struct
        let s_36_7: ProductType887a02170738ab3b = {
            let value = state
                .read_register::<ProductType887a02170738ab3b>(s_36_6 as isize);
            tracer.read_register(s_36_6 as isize, value);
            value
        };
        // C s_36_8: const #91136u : u32
        let s_36_8: u32 = 91136;
        // N s_36_9: write-reg s_36_8 <= s_36_7
        let s_36_9: () = {
            state.write_register::<ProductType887a02170738ab3b>(s_36_8 as isize, s_36_7);
            tracer.write_register(s_36_8 as isize, s_36_7);
        };
        // N s_36_10: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var plshadow#467:i
        let s_37_0: i128 = fn_state.plshadow_467;
        // D s_37_1: call __id(s_37_0)
        let s_37_1: i128 = u__id(state, tracer, s_37_0);
        // C s_37_2: const #1s : i
        let s_37_2: i128 = 1;
        // D s_37_3: sub s_37_1 s_37_2
        let s_37_3: i128 = ((s_37_1) - (s_37_2));
        // C s_37_4: const #256s : i
        let s_37_4: i128 = 256;
        // D s_37_5: cmp-lt s_37_3 s_37_4
        let s_37_5: bool = ((s_37_3) < (s_37_4));
        // D s_37_6: write-var gs#24540 <= s_37_5
        fn_state.gs_24540 = s_37_5;
        // N s_37_7: jump b36
        return block_36(state, tracer, fn_state);
    }
}
