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
use ZeroUnsignedSatQ::*;
use R_read::*;
use u__id::*;
use R_set::*;
use common::*;
pub fn execute_aarch32_instrs_USAT16_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    n: i64,
    saturate_to: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_889054: ProductTypef506aa96a892fe52,
        gs_305030: bool,
        sat2: bool,
        gs_889064: ProductTypef506aa96a892fe52,
        d: i64,
        n: i64,
        saturate_to: i64,
    }
    let fn_state = FunctionState {
        d,
        n,
        saturate_to,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var saturate_to:i64
        let s_0_0: i64 = fn_state.saturate_to;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call __id(s_0_1)
        let s_0_2: i128 = u__id(state, tracer, s_0_1);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // C s_0_4: const #1s : i
        let s_0_4: i128 = 1;
        // D s_0_5: cast zx s_0_3 -> i
        let s_0_5: i128 = (i128::try_from(s_0_3).unwrap());
        // D s_0_6: sub s_0_5 s_0_4
        let s_0_6: i128 = ((s_0_5) - (s_0_4));
        // D s_0_7: cast reint s_0_6 -> i64
        let s_0_7: i64 = (s_0_6 as i64);
        // C s_0_8: const #0s : i
        let s_0_8: i128 = 0;
        // D s_0_9: cast zx s_0_7 -> i
        let s_0_9: i128 = (i128::try_from(s_0_7).unwrap());
        // D s_0_10: cmp-le s_0_8 s_0_9
        let s_0_10: bool = ((s_0_8) <= (s_0_9));
        // N s_0_11: assert s_0_10
        let s_0_11: () = assert!(s_0_10);
        // D s_0_12: read-var n:i64
        let s_0_12: i64 = fn_state.n;
        // D s_0_13: cast zx s_0_12 -> i
        let s_0_13: i128 = (i128::try_from(s_0_12).unwrap());
        // D s_0_14: call R_read(s_0_13)
        let s_0_14: u32 = R_read(state, tracer, s_0_13);
        // C s_0_15: const #0s : i
        let s_0_15: i128 = 0;
        // D s_0_16: cast zx s_0_14 -> bv
        let s_0_16: Bits = Bits::new(s_0_14 as u128, 32u16);
        // C s_0_17: const #1s : i64
        let s_0_17: i64 = 1;
        // C s_0_18: cast zx s_0_17 -> i
        let s_0_18: i128 = (i128::try_from(s_0_17).unwrap());
        // C s_0_19: const #15s : i
        let s_0_19: i128 = 15;
        // C s_0_20: add s_0_19 s_0_18
        let s_0_20: i128 = (s_0_19 + s_0_18);
        // D s_0_21: bit-extract s_0_16 s_0_15 s_0_20
        let s_0_21: Bits = (Bits::new(
            ((s_0_16) >> (s_0_15)).value(),
            u16::try_from(s_0_20).unwrap(),
        ));
        // D s_0_22: cast reint s_0_21 -> u16
        let s_0_22: u16 = (s_0_21.value() as u16);
        // D s_0_23: cast zx s_0_22 -> bv
        let s_0_23: Bits = Bits::new(s_0_22 as u128, 16u16);
        // D s_0_24: cast sx s_0_23 -> i
        let s_0_24: i128 = {
            let sign_bit = s_0_23.length() - 1;
            let mut result = s_0_23.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_0_25: cast reint s_0_24 -> i64
        let s_0_25: i64 = (s_0_24 as i64);
        // C s_0_26: const #16s : i64
        let s_0_26: i64 = 16;
        // D s_0_27: cast zx s_0_25 -> i
        let s_0_27: i128 = (i128::try_from(s_0_25).unwrap());
        // D s_0_28: read-var saturate_to:i64
        let s_0_28: i64 = fn_state.saturate_to;
        // D s_0_29: cast zx s_0_28 -> i
        let s_0_29: i128 = (i128::try_from(s_0_28).unwrap());
        // C s_0_30: cast zx s_0_26 -> i
        let s_0_30: i128 = (i128::try_from(s_0_26).unwrap());
        // D s_0_31: call ZeroUnsignedSatQ(s_0_27, s_0_29, s_0_30)
        let s_0_31: ProductTypef506aa96a892fe52 = ZeroUnsignedSatQ(
            state,
            tracer,
            s_0_27,
            s_0_29,
            s_0_30,
        );
        // D s_0_32: write-var gs#889054 <= s_0_31
        fn_state.gs_889054 = s_0_31;
        // D s_0_33: read-var gs#889054.0:struct
        let s_0_33: Bits = fn_state.gs_889054._0;
        // D s_0_34: cast reint s_0_33 -> u16
        let s_0_34: u16 = (s_0_33.value() as u16);
        // D s_0_35: read-var gs#889054.1:struct
        let s_0_35: bool = fn_state.gs_889054._1;
        // D s_0_36: read-var n:i64
        let s_0_36: i64 = fn_state.n;
        // D s_0_37: cast zx s_0_36 -> i
        let s_0_37: i128 = (i128::try_from(s_0_36).unwrap());
        // D s_0_38: call R_read(s_0_37)
        let s_0_38: u32 = R_read(state, tracer, s_0_37);
        // C s_0_39: const #16s : i
        let s_0_39: i128 = 16;
        // D s_0_40: cast zx s_0_38 -> bv
        let s_0_40: Bits = Bits::new(s_0_38 as u128, 32u16);
        // C s_0_41: const #1s : i64
        let s_0_41: i64 = 1;
        // C s_0_42: cast zx s_0_41 -> i
        let s_0_42: i128 = (i128::try_from(s_0_41).unwrap());
        // C s_0_43: const #15s : i
        let s_0_43: i128 = 15;
        // C s_0_44: add s_0_43 s_0_42
        let s_0_44: i128 = (s_0_43 + s_0_42);
        // D s_0_45: bit-extract s_0_40 s_0_39 s_0_44
        let s_0_45: Bits = (Bits::new(
            ((s_0_40) >> (s_0_39)).value(),
            u16::try_from(s_0_44).unwrap(),
        ));
        // D s_0_46: cast reint s_0_45 -> u16
        let s_0_46: u16 = (s_0_45.value() as u16);
        // D s_0_47: cast zx s_0_46 -> bv
        let s_0_47: Bits = Bits::new(s_0_46 as u128, 16u16);
        // D s_0_48: cast sx s_0_47 -> i
        let s_0_48: i128 = {
            let sign_bit = s_0_47.length() - 1;
            let mut result = s_0_47.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_0_49: cast reint s_0_48 -> i64
        let s_0_49: i64 = (s_0_48 as i64);
        // C s_0_50: const #16s : i64
        let s_0_50: i64 = 16;
        // D s_0_51: cast zx s_0_49 -> i
        let s_0_51: i128 = (i128::try_from(s_0_49).unwrap());
        // D s_0_52: read-var saturate_to:i64
        let s_0_52: i64 = fn_state.saturate_to;
        // D s_0_53: cast zx s_0_52 -> i
        let s_0_53: i128 = (i128::try_from(s_0_52).unwrap());
        // C s_0_54: cast zx s_0_50 -> i
        let s_0_54: i128 = (i128::try_from(s_0_50).unwrap());
        // D s_0_55: call ZeroUnsignedSatQ(s_0_51, s_0_53, s_0_54)
        let s_0_55: ProductTypef506aa96a892fe52 = ZeroUnsignedSatQ(
            state,
            tracer,
            s_0_51,
            s_0_53,
            s_0_54,
        );
        // D s_0_56: write-var gs#889064 <= s_0_55
        fn_state.gs_889064 = s_0_55;
        // D s_0_57: read-var gs#889064.0:struct
        let s_0_57: Bits = fn_state.gs_889064._0;
        // D s_0_58: cast reint s_0_57 -> u16
        let s_0_58: u16 = (s_0_57.value() as u16);
        // D s_0_59: read-var gs#889064.1:struct
        let s_0_59: bool = fn_state.gs_889064._1;
        // D s_0_60: write-var sat2 <= s_0_59
        fn_state.sat2 = s_0_59;
        // D s_0_61: read-var d:i64
        let s_0_61: i64 = fn_state.d;
        // D s_0_62: cast zx s_0_61 -> i
        let s_0_62: i128 = (i128::try_from(s_0_61).unwrap());
        // D s_0_63: call R_read(s_0_62)
        let s_0_63: u32 = R_read(state, tracer, s_0_62);
        // C s_0_64: const #0s : i
        let s_0_64: i128 = 0;
        // D s_0_65: cast zx s_0_63 -> bv
        let s_0_65: Bits = Bits::new(s_0_63 as u128, 32u16);
        // D s_0_66: cast zx s_0_34 -> bv
        let s_0_66: Bits = Bits::new(s_0_34 as u128, 16u16);
        // C s_0_67: const #15s : i
        let s_0_67: i128 = 15;
        // C s_0_68: const #1u : u64
        let s_0_68: u64 = 1;
        // C s_0_69: cast zx s_0_68 -> bv
        let s_0_69: Bits = Bits::new(s_0_68 as u128, 64u16);
        // C s_0_70: lsl s_0_69 s_0_67
        let s_0_70: Bits = s_0_69 << s_0_67;
        // C s_0_71: sub s_0_70 s_0_69
        let s_0_71: Bits = ((s_0_70) - (s_0_69));
        // D s_0_72: and s_0_66 s_0_71
        let s_0_72: Bits = ((s_0_66) & (s_0_71));
        // D s_0_73: lsl s_0_72 s_0_64
        let s_0_73: Bits = s_0_72 << s_0_64;
        // C s_0_74: lsl s_0_71 s_0_64
        let s_0_74: Bits = s_0_71 << s_0_64;
        // C s_0_75: cmpl s_0_74
        let s_0_75: Bits = !s_0_74;
        // D s_0_76: and s_0_65 s_0_75
        let s_0_76: Bits = ((s_0_65) & (s_0_75));
        // D s_0_77: or s_0_76 s_0_73
        let s_0_77: Bits = ((s_0_76) | (s_0_73));
        // D s_0_78: cast reint s_0_77 -> u32
        let s_0_78: u32 = (s_0_77.value() as u32);
        // D s_0_79: read-var d:i64
        let s_0_79: i64 = fn_state.d;
        // D s_0_80: cast zx s_0_79 -> i
        let s_0_80: i128 = (i128::try_from(s_0_79).unwrap());
        // D s_0_81: call R_set(s_0_80, s_0_78)
        let s_0_81: () = R_set(state, tracer, s_0_80, s_0_78);
        // D s_0_82: read-var d:i64
        let s_0_82: i64 = fn_state.d;
        // D s_0_83: cast zx s_0_82 -> i
        let s_0_83: i128 = (i128::try_from(s_0_82).unwrap());
        // D s_0_84: call R_read(s_0_83)
        let s_0_84: u32 = R_read(state, tracer, s_0_83);
        // C s_0_85: const #16s : i
        let s_0_85: i128 = 16;
        // D s_0_86: cast zx s_0_84 -> bv
        let s_0_86: Bits = Bits::new(s_0_84 as u128, 32u16);
        // D s_0_87: cast zx s_0_58 -> bv
        let s_0_87: Bits = Bits::new(s_0_58 as u128, 16u16);
        // C s_0_88: const #15s : i
        let s_0_88: i128 = 15;
        // C s_0_89: const #1u : u64
        let s_0_89: u64 = 1;
        // C s_0_90: cast zx s_0_89 -> bv
        let s_0_90: Bits = Bits::new(s_0_89 as u128, 64u16);
        // C s_0_91: lsl s_0_90 s_0_88
        let s_0_91: Bits = s_0_90 << s_0_88;
        // C s_0_92: sub s_0_91 s_0_90
        let s_0_92: Bits = ((s_0_91) - (s_0_90));
        // D s_0_93: and s_0_87 s_0_92
        let s_0_93: Bits = ((s_0_87) & (s_0_92));
        // D s_0_94: lsl s_0_93 s_0_85
        let s_0_94: Bits = s_0_93 << s_0_85;
        // C s_0_95: lsl s_0_92 s_0_85
        let s_0_95: Bits = s_0_92 << s_0_85;
        // C s_0_96: cmpl s_0_95
        let s_0_96: Bits = !s_0_95;
        // D s_0_97: and s_0_86 s_0_96
        let s_0_97: Bits = ((s_0_86) & (s_0_96));
        // D s_0_98: or s_0_97 s_0_94
        let s_0_98: Bits = ((s_0_97) | (s_0_94));
        // D s_0_99: cast reint s_0_98 -> u32
        let s_0_99: u32 = (s_0_98.value() as u32);
        // D s_0_100: read-var d:i64
        let s_0_100: i64 = fn_state.d;
        // D s_0_101: cast zx s_0_100 -> i
        let s_0_101: i128 = (i128::try_from(s_0_100).unwrap());
        // D s_0_102: call R_set(s_0_101, s_0_99)
        let s_0_102: () = R_set(state, tracer, s_0_101, s_0_99);
        // N s_0_103: branch s_0_35 b5 b1
        if s_0_35 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var sat2:u8
        let s_1_0: bool = fn_state.sat2;
        // D s_1_1: write-var gs#305030 <= s_1_0
        fn_state.gs_305030 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#305030:u8
        let s_2_0: bool = fn_state.gs_305030;
        // N s_2_1: branch s_2_0 b4 b3
        if s_2_0 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #1u : u8
        let s_4_0: bool = true;
        // C s_4_1: const #16988u : u32
        let s_4_1: u32 = 16988;
        // N s_4_2: write-reg s_4_1 <= s_4_0
        let s_4_2: () = {
            state.write_register::<bool>(s_4_1 as isize, s_4_0);
            tracer.write_register(s_4_1 as isize, s_4_0);
        };
        // N s_4_3: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #1u : u8
        let s_5_0: bool = true;
        // D s_5_1: write-var gs#305030 <= s_5_0
        fn_state.gs_305030 = s_5_0;
        // N s_5_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
