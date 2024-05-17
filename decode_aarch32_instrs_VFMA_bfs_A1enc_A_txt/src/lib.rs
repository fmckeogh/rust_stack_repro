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
use HaveAArch32BF16Ext::*;
use execute_aarch32_instrs_VFMA_bfs_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VFMA_bfs_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    Vn: u8,
    Vd: u8,
    N: bool,
    Q: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_327159: bool,
        D: bool,
        Vn: u8,
        Vd: u8,
        N: bool,
        Q: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        Vn,
        Vd,
        N,
        Q,
        M,
        Vm,
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
        // S s_0_1: call HaveAArch32BF16Ext(s_0_0)
        let s_0_1: bool = HaveAArch32BF16Ext(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b7 b1
        if s_0_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0s : i
        let s_1_0: i128 = 0;
        // D s_1_1: read-var Vd:u8
        let s_1_1: u8 = fn_state.Vd;
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 4u16);
        // C s_1_3: const #1u : u64
        let s_1_3: u64 = 1;
        // D s_1_4: bit-extract s_1_2 s_1_0 s_1_3
        let s_1_4: Bits = (Bits::new(
            ((s_1_2) >> (s_1_0)).value(),
            u16::try_from(s_1_3).unwrap(),
        ));
        // D s_1_5: cast reint s_1_4 -> u8
        let s_1_5: bool = ((s_1_4.value()) != 0);
        // C s_1_6: const #0s : i
        let s_1_6: i128 = 0;
        // C s_1_7: const #0u : u64
        let s_1_7: u64 = 0;
        // D s_1_8: cast zx s_1_5 -> u64
        let s_1_8: u64 = (s_1_5 as u64);
        // C s_1_9: const #1u : u64
        let s_1_9: u64 = 1;
        // D s_1_10: and s_1_8 s_1_9
        let s_1_10: u64 = ((s_1_8) & (s_1_9));
        // D s_1_11: cmp-eq s_1_10 s_1_9
        let s_1_11: bool = ((s_1_10) == (s_1_9));
        // D s_1_12: lsl s_1_8 s_1_6
        let s_1_12: u64 = s_1_8 << s_1_6;
        // D s_1_13: or s_1_7 s_1_12
        let s_1_13: u64 = ((s_1_7) | (s_1_12));
        // D s_1_14: cmpl s_1_12
        let s_1_14: u64 = !s_1_12;
        // D s_1_15: and s_1_7 s_1_14
        let s_1_15: u64 = ((s_1_7) & (s_1_14));
        // D s_1_16: select s_1_11 s_1_13 s_1_15
        let s_1_16: u64 = if s_1_11 { s_1_13 } else { s_1_15 };
        // D s_1_17: cast trunc s_1_16 -> u8
        let s_1_17: bool = ((s_1_16) != 0);
        // D s_1_18: cast zx s_1_17 -> bv
        let s_1_18: Bits = Bits::new(s_1_17 as u128, 1u16);
        // C s_1_19: const #1u : u8
        let s_1_19: bool = true;
        // C s_1_20: cast zx s_1_19 -> bv
        let s_1_20: Bits = Bits::new(s_1_19 as u128, 1u16);
        // D s_1_21: cmp-eq s_1_18 s_1_20
        let s_1_21: bool = ((s_1_18) == (s_1_20));
        // N s_1_22: branch s_1_21 b6 b2
        if s_1_21 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0s : i
        let s_2_0: i128 = 0;
        // D s_2_1: read-var Vn:u8
        let s_2_1: u8 = fn_state.Vn;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 4u16);
        // C s_2_3: const #1u : u64
        let s_2_3: u64 = 1;
        // D s_2_4: bit-extract s_2_2 s_2_0 s_2_3
        let s_2_4: Bits = (Bits::new(
            ((s_2_2) >> (s_2_0)).value(),
            u16::try_from(s_2_3).unwrap(),
        ));
        // D s_2_5: cast reint s_2_4 -> u8
        let s_2_5: bool = ((s_2_4.value()) != 0);
        // C s_2_6: const #0s : i
        let s_2_6: i128 = 0;
        // C s_2_7: const #0u : u64
        let s_2_7: u64 = 0;
        // D s_2_8: cast zx s_2_5 -> u64
        let s_2_8: u64 = (s_2_5 as u64);
        // C s_2_9: const #1u : u64
        let s_2_9: u64 = 1;
        // D s_2_10: and s_2_8 s_2_9
        let s_2_10: u64 = ((s_2_8) & (s_2_9));
        // D s_2_11: cmp-eq s_2_10 s_2_9
        let s_2_11: bool = ((s_2_10) == (s_2_9));
        // D s_2_12: lsl s_2_8 s_2_6
        let s_2_12: u64 = s_2_8 << s_2_6;
        // D s_2_13: or s_2_7 s_2_12
        let s_2_13: u64 = ((s_2_7) | (s_2_12));
        // D s_2_14: cmpl s_2_12
        let s_2_14: u64 = !s_2_12;
        // D s_2_15: and s_2_7 s_2_14
        let s_2_15: u64 = ((s_2_7) & (s_2_14));
        // D s_2_16: select s_2_11 s_2_13 s_2_15
        let s_2_16: u64 = if s_2_11 { s_2_13 } else { s_2_15 };
        // D s_2_17: cast trunc s_2_16 -> u8
        let s_2_17: bool = ((s_2_16) != 0);
        // D s_2_18: cast zx s_2_17 -> bv
        let s_2_18: Bits = Bits::new(s_2_17 as u128, 1u16);
        // C s_2_19: const #1u : u8
        let s_2_19: bool = true;
        // C s_2_20: cast zx s_2_19 -> bv
        let s_2_20: Bits = Bits::new(s_2_19 as u128, 1u16);
        // D s_2_21: cmp-eq s_2_18 s_2_20
        let s_2_21: bool = ((s_2_18) == (s_2_20));
        // D s_2_22: write-var gs#327159 <= s_2_21
        fn_state.gs_327159 = s_2_21;
        // N s_2_23: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#327159:u8
        let s_3_0: bool = fn_state.gs_327159;
        // N s_3_1: branch s_3_0 b5 b4
        if s_3_0 {
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
        // D s_4_0: read-var D:u8
        let s_4_0: bool = fn_state.D;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 1u16);
        // D s_4_2: read-var Vd:u8
        let s_4_2: u8 = fn_state.Vd;
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 4u16);
        // D s_4_4: cast reint s_4_1 -> u128
        let s_4_4: u128 = (s_4_1.value() as u128);
        // D s_4_5: size-of s_4_1
        let s_4_5: u16 = s_4_1.length();
        // D s_4_6: cast reint s_4_3 -> u128
        let s_4_6: u128 = (s_4_3.value() as u128);
        // D s_4_7: size-of s_4_3
        let s_4_7: u16 = s_4_3.length();
        // D s_4_8: lsl s_4_4 s_4_7
        let s_4_8: u128 = s_4_4 << s_4_7;
        // D s_4_9: or s_4_8 s_4_6
        let s_4_9: u128 = ((s_4_8) | (s_4_6));
        // D s_4_10: add s_4_5 s_4_7
        let s_4_10: u16 = (s_4_5 + s_4_7);
        // D s_4_11: create-bits s_4_9 s_4_10
        let s_4_11: Bits = Bits::new(s_4_9, s_4_10);
        // D s_4_12: cast reint s_4_11 -> u8
        let s_4_12: u8 = (s_4_11.value() as u8);
        // D s_4_13: cast zx s_4_12 -> bv
        let s_4_13: Bits = Bits::new(s_4_12 as u128, 5u16);
        // D s_4_14: cast zx s_4_13 -> i
        let s_4_14: i128 = (s_4_13.value() as i128);
        // D s_4_15: cast reint s_4_14 -> i64
        let s_4_15: i64 = (s_4_14 as i64);
        // D s_4_16: read-var N:u8
        let s_4_16: bool = fn_state.N;
        // D s_4_17: cast zx s_4_16 -> bv
        let s_4_17: Bits = Bits::new(s_4_16 as u128, 1u16);
        // D s_4_18: read-var Vn:u8
        let s_4_18: u8 = fn_state.Vn;
        // D s_4_19: cast zx s_4_18 -> bv
        let s_4_19: Bits = Bits::new(s_4_18 as u128, 4u16);
        // D s_4_20: cast reint s_4_17 -> u128
        let s_4_20: u128 = (s_4_17.value() as u128);
        // D s_4_21: size-of s_4_17
        let s_4_21: u16 = s_4_17.length();
        // D s_4_22: cast reint s_4_19 -> u128
        let s_4_22: u128 = (s_4_19.value() as u128);
        // D s_4_23: size-of s_4_19
        let s_4_23: u16 = s_4_19.length();
        // D s_4_24: lsl s_4_20 s_4_23
        let s_4_24: u128 = s_4_20 << s_4_23;
        // D s_4_25: or s_4_24 s_4_22
        let s_4_25: u128 = ((s_4_24) | (s_4_22));
        // D s_4_26: add s_4_21 s_4_23
        let s_4_26: u16 = (s_4_21 + s_4_23);
        // D s_4_27: create-bits s_4_25 s_4_26
        let s_4_27: Bits = Bits::new(s_4_25, s_4_26);
        // D s_4_28: cast reint s_4_27 -> u8
        let s_4_28: u8 = (s_4_27.value() as u8);
        // D s_4_29: cast zx s_4_28 -> bv
        let s_4_29: Bits = Bits::new(s_4_28 as u128, 5u16);
        // D s_4_30: cast zx s_4_29 -> i
        let s_4_30: i128 = (s_4_29.value() as i128);
        // D s_4_31: cast reint s_4_30 -> i64
        let s_4_31: i64 = (s_4_30 as i64);
        // C s_4_32: const #0s : i
        let s_4_32: i128 = 0;
        // D s_4_33: read-var Vm:u8
        let s_4_33: u8 = fn_state.Vm;
        // D s_4_34: cast zx s_4_33 -> bv
        let s_4_34: Bits = Bits::new(s_4_33 as u128, 4u16);
        // C s_4_35: const #1s : i64
        let s_4_35: i64 = 1;
        // C s_4_36: cast zx s_4_35 -> i
        let s_4_36: i128 = (i128::try_from(s_4_35).unwrap());
        // C s_4_37: const #2s : i
        let s_4_37: i128 = 2;
        // C s_4_38: add s_4_37 s_4_36
        let s_4_38: i128 = (s_4_37 + s_4_36);
        // D s_4_39: bit-extract s_4_34 s_4_32 s_4_38
        let s_4_39: Bits = (Bits::new(
            ((s_4_34) >> (s_4_32)).value(),
            u16::try_from(s_4_38).unwrap(),
        ));
        // D s_4_40: cast reint s_4_39 -> u8
        let s_4_40: u8 = (s_4_39.value() as u8);
        // D s_4_41: cast zx s_4_40 -> bv
        let s_4_41: Bits = Bits::new(s_4_40 as u128, 3u16);
        // D s_4_42: cast zx s_4_41 -> i
        let s_4_42: i128 = (s_4_41.value() as i128);
        // D s_4_43: cast reint s_4_42 -> i64
        let s_4_43: i64 = (s_4_42 as i64);
        // C s_4_44: const #3s : i
        let s_4_44: i128 = 3;
        // D s_4_45: read-var Vm:u8
        let s_4_45: u8 = fn_state.Vm;
        // D s_4_46: cast zx s_4_45 -> bv
        let s_4_46: Bits = Bits::new(s_4_45 as u128, 4u16);
        // C s_4_47: const #1u : u64
        let s_4_47: u64 = 1;
        // D s_4_48: bit-extract s_4_46 s_4_44 s_4_47
        let s_4_48: Bits = (Bits::new(
            ((s_4_46) >> (s_4_44)).value(),
            u16::try_from(s_4_47).unwrap(),
        ));
        // D s_4_49: cast reint s_4_48 -> u8
        let s_4_49: bool = ((s_4_48.value()) != 0);
        // C s_4_50: const #0s : i
        let s_4_50: i128 = 0;
        // C s_4_51: const #0u : u64
        let s_4_51: u64 = 0;
        // D s_4_52: cast zx s_4_49 -> u64
        let s_4_52: u64 = (s_4_49 as u64);
        // C s_4_53: const #1u : u64
        let s_4_53: u64 = 1;
        // D s_4_54: and s_4_52 s_4_53
        let s_4_54: u64 = ((s_4_52) & (s_4_53));
        // D s_4_55: cmp-eq s_4_54 s_4_53
        let s_4_55: bool = ((s_4_54) == (s_4_53));
        // D s_4_56: lsl s_4_52 s_4_50
        let s_4_56: u64 = s_4_52 << s_4_50;
        // D s_4_57: or s_4_51 s_4_56
        let s_4_57: u64 = ((s_4_51) | (s_4_56));
        // D s_4_58: cmpl s_4_56
        let s_4_58: u64 = !s_4_56;
        // D s_4_59: and s_4_51 s_4_58
        let s_4_59: u64 = ((s_4_51) & (s_4_58));
        // D s_4_60: select s_4_55 s_4_57 s_4_59
        let s_4_60: u64 = if s_4_55 { s_4_57 } else { s_4_59 };
        // D s_4_61: cast trunc s_4_60 -> u8
        let s_4_61: bool = ((s_4_60) != 0);
        // D s_4_62: read-var M:u8
        let s_4_62: bool = fn_state.M;
        // D s_4_63: cast zx s_4_62 -> bv
        let s_4_63: Bits = Bits::new(s_4_62 as u128, 1u16);
        // D s_4_64: cast zx s_4_61 -> bv
        let s_4_64: Bits = Bits::new(s_4_61 as u128, 1u16);
        // D s_4_65: cast reint s_4_63 -> u128
        let s_4_65: u128 = (s_4_63.value() as u128);
        // D s_4_66: size-of s_4_63
        let s_4_66: u16 = s_4_63.length();
        // D s_4_67: cast reint s_4_64 -> u128
        let s_4_67: u128 = (s_4_64.value() as u128);
        // D s_4_68: size-of s_4_64
        let s_4_68: u16 = s_4_64.length();
        // D s_4_69: lsl s_4_65 s_4_68
        let s_4_69: u128 = s_4_65 << s_4_68;
        // D s_4_70: or s_4_69 s_4_67
        let s_4_70: u128 = ((s_4_69) | (s_4_67));
        // D s_4_71: add s_4_66 s_4_68
        let s_4_71: u16 = (s_4_66 + s_4_68);
        // D s_4_72: create-bits s_4_70 s_4_71
        let s_4_72: Bits = Bits::new(s_4_70, s_4_71);
        // D s_4_73: cast reint s_4_72 -> u8
        let s_4_73: u8 = (s_4_72.value() as u8);
        // D s_4_74: cast zx s_4_73 -> bv
        let s_4_74: Bits = Bits::new(s_4_73 as u128, 2u16);
        // D s_4_75: cast zx s_4_74 -> i
        let s_4_75: i128 = (s_4_74.value() as i128);
        // D s_4_76: cast reint s_4_75 -> i64
        let s_4_76: i64 = (s_4_75 as i64);
        // C s_4_77: const #4s : i64
        let s_4_77: i64 = 4;
        // D s_4_78: read-var Q:u8
        let s_4_78: bool = fn_state.Q;
        // D s_4_79: cast zx s_4_78 -> bv
        let s_4_79: Bits = Bits::new(s_4_78 as u128, 1u16);
        // D s_4_80: cast zx s_4_79 -> i
        let s_4_80: i128 = (s_4_79.value() as i128);
        // D s_4_81: cast reint s_4_80 -> i64
        let s_4_81: i64 = (s_4_80 as i64);
        // D s_4_82: call execute_aarch32_instrs_VFMA_bfs_Op_A_txt(s_4_15, s_4_77, s_4_76, s_4_43, s_4_31, s_4_81)
        let s_4_82: () = execute_aarch32_instrs_VFMA_bfs_Op_A_txt(
            state,
            tracer,
            s_4_15,
            s_4_77,
            s_4_76,
            s_4_43,
            s_4_31,
            s_4_81,
        );
        // N s_4_83: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: panic
        panic!("{:?}", ());
        // N s_5_1: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #1u : u8
        let s_6_0: bool = true;
        // D s_6_1: write-var gs#327159 <= s_6_0
        fn_state.gs_327159 = s_6_0;
        // N s_6_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: panic
        panic!("{:?}", ());
        // N s_7_1: return
        return;
    }
}
