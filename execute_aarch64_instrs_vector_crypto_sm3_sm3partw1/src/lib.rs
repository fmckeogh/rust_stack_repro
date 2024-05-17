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
use ROL::*;
use V_set::*;
use V_read::*;
use AArch64_CheckFPAdvSIMDEnabled::*;
use common::*;
pub fn execute_aarch64_instrs_vector_crypto_sm3_sm3partw1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        Vn: u128,
        Vd: u128,
        result: u128,
        i: i64,
        d: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        m,
        n,
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
        // S s_0_1: call AArch64_CheckFPAdvSIMDEnabled(s_0_0)
        let s_0_1: () = AArch64_CheckFPAdvSIMDEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #128s : i64
        let s_1_0: i64 = 128;
        // D s_1_1: read-var m:i64
        let s_1_1: i64 = fn_state.m;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: call V_read(s_1_2, s_1_0)
        let s_1_3: Bits = V_read(state, tracer, s_1_2, s_1_0);
        // D s_1_4: cast reint s_1_3 -> u128
        let s_1_4: u128 = (s_1_3.value() as u128);
        // C s_1_5: const #128s : i64
        let s_1_5: i64 = 128;
        // D s_1_6: read-var n:i64
        let s_1_6: i64 = fn_state.n;
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_8: call V_read(s_1_7, s_1_5)
        let s_1_8: Bits = V_read(state, tracer, s_1_7, s_1_5);
        // D s_1_9: cast reint s_1_8 -> u128
        let s_1_9: u128 = (s_1_8.value() as u128);
        // D s_1_10: write-var Vn <= s_1_9
        fn_state.Vn = s_1_9;
        // C s_1_11: const #128s : i64
        let s_1_11: i64 = 128;
        // D s_1_12: read-var d:i64
        let s_1_12: i64 = fn_state.d;
        // D s_1_13: cast zx s_1_12 -> i
        let s_1_13: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_14: call V_read(s_1_13, s_1_11)
        let s_1_14: Bits = V_read(state, tracer, s_1_13, s_1_11);
        // D s_1_15: cast reint s_1_14 -> u128
        let s_1_15: u128 = (s_1_14.value() as u128);
        // D s_1_16: write-var Vd <= s_1_15
        fn_state.Vd = s_1_15;
        // D s_1_17: read-var Vd:u128
        let s_1_17: u128 = fn_state.Vd;
        // D s_1_18: cast zx s_1_17 -> bv
        let s_1_18: Bits = Bits::new(s_1_17 as u128, 128u16);
        // D s_1_19: read-var Vn:u128
        let s_1_19: u128 = fn_state.Vn;
        // D s_1_20: cast zx s_1_19 -> bv
        let s_1_20: Bits = Bits::new(s_1_19 as u128, 128u16);
        // D s_1_21: xor s_1_18 s_1_20
        let s_1_21: Bits = ((s_1_18) ^ (s_1_20));
        // D s_1_22: cast reint s_1_21 -> u128
        let s_1_22: u128 = (s_1_21.value() as u128);
        // C s_1_23: const #0s : i
        let s_1_23: i128 = 0;
        // D s_1_24: cast zx s_1_22 -> bv
        let s_1_24: Bits = Bits::new(s_1_22 as u128, 128u16);
        // C s_1_25: const #1s : i64
        let s_1_25: i64 = 1;
        // C s_1_26: cast zx s_1_25 -> i
        let s_1_26: i128 = (i128::try_from(s_1_25).unwrap());
        // C s_1_27: const #95s : i
        let s_1_27: i128 = 95;
        // C s_1_28: add s_1_27 s_1_26
        let s_1_28: i128 = (s_1_27 + s_1_26);
        // D s_1_29: bit-extract s_1_24 s_1_23 s_1_28
        let s_1_29: Bits = (Bits::new(
            ((s_1_24) >> (s_1_23)).value(),
            u16::try_from(s_1_28).unwrap(),
        ));
        // D s_1_30: cast reint s_1_29 -> u96
        let s_1_30: u128 = (s_1_29.value() as u128);
        // C s_1_31: const #96s : i
        let s_1_31: i128 = 96;
        // D s_1_32: cast zx s_1_4 -> bv
        let s_1_32: Bits = Bits::new(s_1_4 as u128, 128u16);
        // C s_1_33: const #1s : i64
        let s_1_33: i64 = 1;
        // C s_1_34: cast zx s_1_33 -> i
        let s_1_34: i128 = (i128::try_from(s_1_33).unwrap());
        // C s_1_35: const #31s : i
        let s_1_35: i128 = 31;
        // C s_1_36: add s_1_35 s_1_34
        let s_1_36: i128 = (s_1_35 + s_1_34);
        // D s_1_37: bit-extract s_1_32 s_1_31 s_1_36
        let s_1_37: Bits = (Bits::new(
            ((s_1_32) >> (s_1_31)).value(),
            u16::try_from(s_1_36).unwrap(),
        ));
        // D s_1_38: cast reint s_1_37 -> u32
        let s_1_38: u32 = (s_1_37.value() as u32);
        // C s_1_39: const #15s : i
        let s_1_39: i128 = 15;
        // D s_1_40: cast zx s_1_38 -> bv
        let s_1_40: Bits = Bits::new(s_1_38 as u128, 32u16);
        // D s_1_41: call ROL(s_1_40, s_1_39)
        let s_1_41: Bits = ROL(state, tracer, s_1_40, s_1_39);
        // D s_1_42: cast reint s_1_41 -> u32
        let s_1_42: u32 = (s_1_41.value() as u32);
        // C s_1_43: const #64s : i
        let s_1_43: i128 = 64;
        // D s_1_44: cast zx s_1_4 -> bv
        let s_1_44: Bits = Bits::new(s_1_4 as u128, 128u16);
        // C s_1_45: const #1s : i64
        let s_1_45: i64 = 1;
        // C s_1_46: cast zx s_1_45 -> i
        let s_1_46: i128 = (i128::try_from(s_1_45).unwrap());
        // C s_1_47: const #31s : i
        let s_1_47: i128 = 31;
        // C s_1_48: add s_1_47 s_1_46
        let s_1_48: i128 = (s_1_47 + s_1_46);
        // D s_1_49: bit-extract s_1_44 s_1_43 s_1_48
        let s_1_49: Bits = (Bits::new(
            ((s_1_44) >> (s_1_43)).value(),
            u16::try_from(s_1_48).unwrap(),
        ));
        // D s_1_50: cast reint s_1_49 -> u32
        let s_1_50: u32 = (s_1_49.value() as u32);
        // C s_1_51: const #15s : i
        let s_1_51: i128 = 15;
        // D s_1_52: cast zx s_1_50 -> bv
        let s_1_52: Bits = Bits::new(s_1_50 as u128, 32u16);
        // D s_1_53: call ROL(s_1_52, s_1_51)
        let s_1_53: Bits = ROL(state, tracer, s_1_52, s_1_51);
        // D s_1_54: cast reint s_1_53 -> u32
        let s_1_54: u32 = (s_1_53.value() as u32);
        // D s_1_55: cast zx s_1_42 -> bv
        let s_1_55: Bits = Bits::new(s_1_42 as u128, 32u16);
        // D s_1_56: cast zx s_1_54 -> bv
        let s_1_56: Bits = Bits::new(s_1_54 as u128, 32u16);
        // D s_1_57: cast reint s_1_55 -> u128
        let s_1_57: u128 = (s_1_55.value() as u128);
        // D s_1_58: size-of s_1_55
        let s_1_58: u16 = s_1_55.length();
        // D s_1_59: cast reint s_1_56 -> u128
        let s_1_59: u128 = (s_1_56.value() as u128);
        // D s_1_60: size-of s_1_56
        let s_1_60: u16 = s_1_56.length();
        // D s_1_61: lsl s_1_57 s_1_60
        let s_1_61: u128 = s_1_57 << s_1_60;
        // D s_1_62: or s_1_61 s_1_59
        let s_1_62: u128 = ((s_1_61) | (s_1_59));
        // D s_1_63: add s_1_58 s_1_60
        let s_1_63: u16 = (s_1_58 + s_1_60);
        // D s_1_64: create-bits s_1_62 s_1_63
        let s_1_64: Bits = Bits::new(s_1_62, s_1_63);
        // D s_1_65: cast reint s_1_64 -> u64
        let s_1_65: u64 = (s_1_64.value() as u64);
        // C s_1_66: const #32s : i
        let s_1_66: i128 = 32;
        // D s_1_67: cast zx s_1_4 -> bv
        let s_1_67: Bits = Bits::new(s_1_4 as u128, 128u16);
        // C s_1_68: const #1s : i64
        let s_1_68: i64 = 1;
        // C s_1_69: cast zx s_1_68 -> i
        let s_1_69: i128 = (i128::try_from(s_1_68).unwrap());
        // C s_1_70: const #31s : i
        let s_1_70: i128 = 31;
        // C s_1_71: add s_1_70 s_1_69
        let s_1_71: i128 = (s_1_70 + s_1_69);
        // D s_1_72: bit-extract s_1_67 s_1_66 s_1_71
        let s_1_72: Bits = (Bits::new(
            ((s_1_67) >> (s_1_66)).value(),
            u16::try_from(s_1_71).unwrap(),
        ));
        // D s_1_73: cast reint s_1_72 -> u32
        let s_1_73: u32 = (s_1_72.value() as u32);
        // C s_1_74: const #15s : i
        let s_1_74: i128 = 15;
        // D s_1_75: cast zx s_1_73 -> bv
        let s_1_75: Bits = Bits::new(s_1_73 as u128, 32u16);
        // D s_1_76: call ROL(s_1_75, s_1_74)
        let s_1_76: Bits = ROL(state, tracer, s_1_75, s_1_74);
        // D s_1_77: cast reint s_1_76 -> u32
        let s_1_77: u32 = (s_1_76.value() as u32);
        // D s_1_78: cast zx s_1_65 -> bv
        let s_1_78: Bits = Bits::new(s_1_65 as u128, 64u16);
        // D s_1_79: cast zx s_1_77 -> bv
        let s_1_79: Bits = Bits::new(s_1_77 as u128, 32u16);
        // D s_1_80: cast reint s_1_78 -> u128
        let s_1_80: u128 = (s_1_78.value() as u128);
        // D s_1_81: size-of s_1_78
        let s_1_81: u16 = s_1_78.length();
        // D s_1_82: cast reint s_1_79 -> u128
        let s_1_82: u128 = (s_1_79.value() as u128);
        // D s_1_83: size-of s_1_79
        let s_1_83: u16 = s_1_79.length();
        // D s_1_84: lsl s_1_80 s_1_83
        let s_1_84: u128 = s_1_80 << s_1_83;
        // D s_1_85: or s_1_84 s_1_82
        let s_1_85: u128 = ((s_1_84) | (s_1_82));
        // D s_1_86: add s_1_81 s_1_83
        let s_1_86: u16 = (s_1_81 + s_1_83);
        // D s_1_87: create-bits s_1_85 s_1_86
        let s_1_87: Bits = Bits::new(s_1_85, s_1_86);
        // D s_1_88: cast reint s_1_87 -> u96
        let s_1_88: u128 = (s_1_87.value() as u128);
        // D s_1_89: cast zx s_1_30 -> bv
        let s_1_89: Bits = Bits::new(s_1_30 as u128, 96u16);
        // D s_1_90: cast zx s_1_88 -> bv
        let s_1_90: Bits = Bits::new(s_1_88 as u128, 96u16);
        // D s_1_91: xor s_1_89 s_1_90
        let s_1_91: Bits = ((s_1_89) ^ (s_1_90));
        // D s_1_92: cast reint s_1_91 -> u96
        let s_1_92: u128 = (s_1_91.value() as u128);
        // C s_1_93: const #0s : i
        let s_1_93: i128 = 0;
        // D s_1_94: read-var result:u128
        let s_1_94: u128 = fn_state.result;
        // D s_1_95: cast zx s_1_94 -> bv
        let s_1_95: Bits = Bits::new(s_1_94 as u128, 128u16);
        // D s_1_96: cast zx s_1_92 -> bv
        let s_1_96: Bits = Bits::new(s_1_92 as u128, 96u16);
        // C s_1_97: const #95s : i
        let s_1_97: i128 = 95;
        // C s_1_98: const #1u : u64
        let s_1_98: u64 = 1;
        // C s_1_99: cast zx s_1_98 -> bv
        let s_1_99: Bits = Bits::new(s_1_98 as u128, 64u16);
        // C s_1_100: lsl s_1_99 s_1_97
        let s_1_100: Bits = s_1_99 << s_1_97;
        // C s_1_101: sub s_1_100 s_1_99
        let s_1_101: Bits = ((s_1_100) - (s_1_99));
        // D s_1_102: and s_1_96 s_1_101
        let s_1_102: Bits = ((s_1_96) & (s_1_101));
        // D s_1_103: lsl s_1_102 s_1_93
        let s_1_103: Bits = s_1_102 << s_1_93;
        // C s_1_104: lsl s_1_101 s_1_93
        let s_1_104: Bits = s_1_101 << s_1_93;
        // C s_1_105: cmpl s_1_104
        let s_1_105: Bits = !s_1_104;
        // D s_1_106: and s_1_95 s_1_105
        let s_1_106: Bits = ((s_1_95) & (s_1_105));
        // D s_1_107: or s_1_106 s_1_103
        let s_1_107: Bits = ((s_1_106) | (s_1_103));
        // D s_1_108: cast reint s_1_107 -> u128
        let s_1_108: u128 = (s_1_107.value() as u128);
        // D s_1_109: write-var result <= s_1_108
        fn_state.result = s_1_108;
        // C s_1_110: const #0s : i64
        let s_1_110: i64 = 0;
        // D s_1_111: write-var i <= s_1_110
        fn_state.i = s_1_110;
        // N s_1_112: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var i:i64
        let s_2_0: i64 = fn_state.i;
        // C s_2_1: const #3s : i64
        let s_2_1: i64 = 3;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b7 b3
        if s_2_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #3s : i
        let s_3_0: i128 = 3;
        // D s_3_1: read-var i:i64
        let s_3_1: i64 = fn_state.i;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: cmp-eq s_3_2 s_3_0
        let s_3_3: bool = ((s_3_2) == (s_3_0));
        // N s_3_4: branch s_3_3 b6 b4
        if s_3_3 {
            return block_6(state, tracer, fn_state);
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
        // C s_5_0: const #32s : i
        let s_5_0: i128 = 32;
        // D s_5_1: read-var i:i64
        let s_5_1: i64 = fn_state.i;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: mul s_5_0 s_5_2
        let s_5_3: i128 = ((s_5_0) * (s_5_2));
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // C s_5_5: const #31s : i
        let s_5_5: i128 = 31;
        // D s_5_6: cast zx s_5_4 -> i
        let s_5_6: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_7: add s_5_6 s_5_5
        let s_5_7: i128 = (s_5_6 + s_5_5);
        // D s_5_8: cast reint s_5_7 -> i64
        let s_5_8: i64 = (s_5_7 as i64);
        // C s_5_9: const #32s : i
        let s_5_9: i128 = 32;
        // D s_5_10: read-var i:i64
        let s_5_10: i64 = fn_state.i;
        // D s_5_11: cast zx s_5_10 -> i
        let s_5_11: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_12: mul s_5_9 s_5_11
        let s_5_12: i128 = ((s_5_9) * (s_5_11));
        // D s_5_13: cast reint s_5_12 -> i64
        let s_5_13: i64 = (s_5_12 as i64);
        // C s_5_14: const #32s : i
        let s_5_14: i128 = 32;
        // D s_5_15: read-var i:i64
        let s_5_15: i64 = fn_state.i;
        // D s_5_16: cast zx s_5_15 -> i
        let s_5_16: i128 = (i128::try_from(s_5_15).unwrap());
        // D s_5_17: mul s_5_14 s_5_16
        let s_5_17: i128 = ((s_5_14) * (s_5_16));
        // D s_5_18: cast reint s_5_17 -> i64
        let s_5_18: i64 = (s_5_17 as i64);
        // C s_5_19: const #32s : i
        let s_5_19: i128 = 32;
        // D s_5_20: read-var result:u128
        let s_5_20: u128 = fn_state.result;
        // D s_5_21: cast zx s_5_20 -> bv
        let s_5_21: Bits = Bits::new(s_5_20 as u128, 128u16);
        // D s_5_22: cast zx s_5_18 -> i
        let s_5_22: i128 = (i128::try_from(s_5_18).unwrap());
        // D s_5_23: bit-extract s_5_21 s_5_22 s_5_19
        let s_5_23: Bits = (Bits::new(
            ((s_5_21) >> (s_5_22)).value(),
            u16::try_from(s_5_19).unwrap(),
        ));
        // D s_5_24: cast reint s_5_23 -> u32
        let s_5_24: u32 = (s_5_23.value() as u32);
        // C s_5_25: const #32s : i
        let s_5_25: i128 = 32;
        // D s_5_26: read-var i:i64
        let s_5_26: i64 = fn_state.i;
        // D s_5_27: cast zx s_5_26 -> i
        let s_5_27: i128 = (i128::try_from(s_5_26).unwrap());
        // D s_5_28: mul s_5_25 s_5_27
        let s_5_28: i128 = ((s_5_25) * (s_5_27));
        // D s_5_29: cast reint s_5_28 -> i64
        let s_5_29: i64 = (s_5_28 as i64);
        // C s_5_30: const #32s : i
        let s_5_30: i128 = 32;
        // D s_5_31: read-var result:u128
        let s_5_31: u128 = fn_state.result;
        // D s_5_32: cast zx s_5_31 -> bv
        let s_5_32: Bits = Bits::new(s_5_31 as u128, 128u16);
        // D s_5_33: cast zx s_5_29 -> i
        let s_5_33: i128 = (i128::try_from(s_5_29).unwrap());
        // D s_5_34: bit-extract s_5_32 s_5_33 s_5_30
        let s_5_34: Bits = (Bits::new(
            ((s_5_32) >> (s_5_33)).value(),
            u16::try_from(s_5_30).unwrap(),
        ));
        // D s_5_35: cast reint s_5_34 -> u32
        let s_5_35: u32 = (s_5_34.value() as u32);
        // C s_5_36: const #15s : i
        let s_5_36: i128 = 15;
        // D s_5_37: cast zx s_5_35 -> bv
        let s_5_37: Bits = Bits::new(s_5_35 as u128, 32u16);
        // D s_5_38: call ROL(s_5_37, s_5_36)
        let s_5_38: Bits = ROL(state, tracer, s_5_37, s_5_36);
        // D s_5_39: cast zx s_5_24 -> bv
        let s_5_39: Bits = Bits::new(s_5_24 as u128, 32u16);
        // D s_5_40: xor s_5_39 s_5_38
        let s_5_40: Bits = ((s_5_39) ^ (s_5_38));
        // C s_5_41: const #32s : i
        let s_5_41: i128 = 32;
        // D s_5_42: read-var i:i64
        let s_5_42: i64 = fn_state.i;
        // D s_5_43: cast zx s_5_42 -> i
        let s_5_43: i128 = (i128::try_from(s_5_42).unwrap());
        // D s_5_44: mul s_5_41 s_5_43
        let s_5_44: i128 = ((s_5_41) * (s_5_43));
        // D s_5_45: cast reint s_5_44 -> i64
        let s_5_45: i64 = (s_5_44 as i64);
        // C s_5_46: const #32s : i
        let s_5_46: i128 = 32;
        // D s_5_47: read-var result:u128
        let s_5_47: u128 = fn_state.result;
        // D s_5_48: cast zx s_5_47 -> bv
        let s_5_48: Bits = Bits::new(s_5_47 as u128, 128u16);
        // D s_5_49: cast zx s_5_45 -> i
        let s_5_49: i128 = (i128::try_from(s_5_45).unwrap());
        // D s_5_50: bit-extract s_5_48 s_5_49 s_5_46
        let s_5_50: Bits = (Bits::new(
            ((s_5_48) >> (s_5_49)).value(),
            u16::try_from(s_5_46).unwrap(),
        ));
        // D s_5_51: cast reint s_5_50 -> u32
        let s_5_51: u32 = (s_5_50.value() as u32);
        // C s_5_52: const #23s : i
        let s_5_52: i128 = 23;
        // D s_5_53: cast zx s_5_51 -> bv
        let s_5_53: Bits = Bits::new(s_5_51 as u128, 32u16);
        // D s_5_54: call ROL(s_5_53, s_5_52)
        let s_5_54: Bits = ROL(state, tracer, s_5_53, s_5_52);
        // D s_5_55: xor s_5_40 s_5_54
        let s_5_55: Bits = ((s_5_40) ^ (s_5_54));
        // D s_5_56: read-var result:u128
        let s_5_56: u128 = fn_state.result;
        // D s_5_57: cast zx s_5_56 -> bv
        let s_5_57: Bits = Bits::new(s_5_56 as u128, 128u16);
        // D s_5_58: cast zx s_5_8 -> i
        let s_5_58: i128 = (i128::try_from(s_5_8).unwrap());
        // D s_5_59: cast zx s_5_13 -> i
        let s_5_59: i128 = (i128::try_from(s_5_13).unwrap());
        // D s_5_60: sub s_5_58 s_5_59
        let s_5_60: i128 = ((s_5_58) - (s_5_59));
        // C s_5_61: const #1u : u64
        let s_5_61: u64 = 1;
        // C s_5_62: cast zx s_5_61 -> bv
        let s_5_62: Bits = Bits::new(s_5_61 as u128, 64u16);
        // D s_5_63: lsl s_5_62 s_5_60
        let s_5_63: Bits = s_5_62 << s_5_60;
        // D s_5_64: sub s_5_63 s_5_62
        let s_5_64: Bits = ((s_5_63) - (s_5_62));
        // D s_5_65: and s_5_55 s_5_64
        let s_5_65: Bits = ((s_5_55) & (s_5_64));
        // D s_5_66: lsl s_5_65 s_5_59
        let s_5_66: Bits = s_5_65 << s_5_59;
        // D s_5_67: lsl s_5_64 s_5_59
        let s_5_67: Bits = s_5_64 << s_5_59;
        // D s_5_68: cmpl s_5_67
        let s_5_68: Bits = !s_5_67;
        // D s_5_69: and s_5_57 s_5_68
        let s_5_69: Bits = ((s_5_57) & (s_5_68));
        // D s_5_70: or s_5_69 s_5_66
        let s_5_70: Bits = ((s_5_69) | (s_5_66));
        // D s_5_71: cast reint s_5_70 -> u128
        let s_5_71: u128 = (s_5_70.value() as u128);
        // D s_5_72: write-var result <= s_5_71
        fn_state.result = s_5_71;
        // D s_5_73: read-var i:i64
        let s_5_73: i64 = fn_state.i;
        // C s_5_74: const #1s : i64
        let s_5_74: i64 = 1;
        // D s_5_75: add s_5_73 s_5_74
        let s_5_75: i64 = (s_5_73 + s_5_74);
        // D s_5_76: write-var i <= s_5_75
        fn_state.i = s_5_75;
        // N s_5_77: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var Vd:u128
        let s_6_0: u128 = fn_state.Vd;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 128u16);
        // D s_6_2: read-var Vn:u128
        let s_6_2: u128 = fn_state.Vn;
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 128u16);
        // D s_6_4: xor s_6_1 s_6_3
        let s_6_4: Bits = ((s_6_1) ^ (s_6_3));
        // D s_6_5: cast reint s_6_4 -> u128
        let s_6_5: u128 = (s_6_4.value() as u128);
        // C s_6_6: const #96s : i
        let s_6_6: i128 = 96;
        // D s_6_7: cast zx s_6_5 -> bv
        let s_6_7: Bits = Bits::new(s_6_5 as u128, 128u16);
        // C s_6_8: const #1s : i64
        let s_6_8: i64 = 1;
        // C s_6_9: cast zx s_6_8 -> i
        let s_6_9: i128 = (i128::try_from(s_6_8).unwrap());
        // C s_6_10: const #31s : i
        let s_6_10: i128 = 31;
        // C s_6_11: add s_6_10 s_6_9
        let s_6_11: i128 = (s_6_10 + s_6_9);
        // D s_6_12: bit-extract s_6_7 s_6_6 s_6_11
        let s_6_12: Bits = (Bits::new(
            ((s_6_7) >> (s_6_6)).value(),
            u16::try_from(s_6_11).unwrap(),
        ));
        // D s_6_13: cast reint s_6_12 -> u32
        let s_6_13: u32 = (s_6_12.value() as u32);
        // C s_6_14: const #0s : i
        let s_6_14: i128 = 0;
        // D s_6_15: read-var result:u128
        let s_6_15: u128 = fn_state.result;
        // D s_6_16: cast zx s_6_15 -> bv
        let s_6_16: Bits = Bits::new(s_6_15 as u128, 128u16);
        // C s_6_17: const #1s : i64
        let s_6_17: i64 = 1;
        // C s_6_18: cast zx s_6_17 -> i
        let s_6_18: i128 = (i128::try_from(s_6_17).unwrap());
        // C s_6_19: const #31s : i
        let s_6_19: i128 = 31;
        // C s_6_20: add s_6_19 s_6_18
        let s_6_20: i128 = (s_6_19 + s_6_18);
        // D s_6_21: bit-extract s_6_16 s_6_14 s_6_20
        let s_6_21: Bits = (Bits::new(
            ((s_6_16) >> (s_6_14)).value(),
            u16::try_from(s_6_20).unwrap(),
        ));
        // D s_6_22: cast reint s_6_21 -> u32
        let s_6_22: u32 = (s_6_21.value() as u32);
        // C s_6_23: const #15s : i
        let s_6_23: i128 = 15;
        // D s_6_24: cast zx s_6_22 -> bv
        let s_6_24: Bits = Bits::new(s_6_22 as u128, 32u16);
        // D s_6_25: call ROL(s_6_24, s_6_23)
        let s_6_25: Bits = ROL(state, tracer, s_6_24, s_6_23);
        // D s_6_26: cast reint s_6_25 -> u32
        let s_6_26: u32 = (s_6_25.value() as u32);
        // D s_6_27: cast zx s_6_13 -> bv
        let s_6_27: Bits = Bits::new(s_6_13 as u128, 32u16);
        // D s_6_28: cast zx s_6_26 -> bv
        let s_6_28: Bits = Bits::new(s_6_26 as u128, 32u16);
        // D s_6_29: xor s_6_27 s_6_28
        let s_6_29: Bits = ((s_6_27) ^ (s_6_28));
        // D s_6_30: cast reint s_6_29 -> u32
        let s_6_30: u32 = (s_6_29.value() as u32);
        // C s_6_31: const #96s : i
        let s_6_31: i128 = 96;
        // D s_6_32: read-var result:u128
        let s_6_32: u128 = fn_state.result;
        // D s_6_33: cast zx s_6_32 -> bv
        let s_6_33: Bits = Bits::new(s_6_32 as u128, 128u16);
        // D s_6_34: cast zx s_6_30 -> bv
        let s_6_34: Bits = Bits::new(s_6_30 as u128, 32u16);
        // C s_6_35: const #31s : i
        let s_6_35: i128 = 31;
        // C s_6_36: const #1u : u64
        let s_6_36: u64 = 1;
        // C s_6_37: cast zx s_6_36 -> bv
        let s_6_37: Bits = Bits::new(s_6_36 as u128, 64u16);
        // C s_6_38: lsl s_6_37 s_6_35
        let s_6_38: Bits = s_6_37 << s_6_35;
        // C s_6_39: sub s_6_38 s_6_37
        let s_6_39: Bits = ((s_6_38) - (s_6_37));
        // D s_6_40: and s_6_34 s_6_39
        let s_6_40: Bits = ((s_6_34) & (s_6_39));
        // D s_6_41: lsl s_6_40 s_6_31
        let s_6_41: Bits = s_6_40 << s_6_31;
        // C s_6_42: lsl s_6_39 s_6_31
        let s_6_42: Bits = s_6_39 << s_6_31;
        // C s_6_43: cmpl s_6_42
        let s_6_43: Bits = !s_6_42;
        // D s_6_44: and s_6_33 s_6_43
        let s_6_44: Bits = ((s_6_33) & (s_6_43));
        // D s_6_45: or s_6_44 s_6_41
        let s_6_45: Bits = ((s_6_44) | (s_6_41));
        // D s_6_46: cast reint s_6_45 -> u128
        let s_6_46: u128 = (s_6_45.value() as u128);
        // D s_6_47: write-var result <= s_6_46
        fn_state.result = s_6_46;
        // N s_6_48: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #128s : i64
        let s_7_0: i64 = 128;
        // D s_7_1: read-var d:i64
        let s_7_1: i64 = fn_state.d;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: read-var result:u128
        let s_7_3: u128 = fn_state.result;
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 128u16);
        // D s_7_5: call V_set(s_7_2, s_7_0, s_7_4)
        let s_7_5: () = V_set(state, tracer, s_7_2, s_7_0, s_7_4);
        // N s_7_6: return
        return;
    }
}
