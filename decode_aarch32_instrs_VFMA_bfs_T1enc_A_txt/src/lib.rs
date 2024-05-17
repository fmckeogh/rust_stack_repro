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
use execute_aarch32_instrs_VFMA_bfs_Op_A_txt::*;
use InITBlock::*;
use HaveAArch32BF16Ext::*;
use common::*;
pub fn decode_aarch32_instrs_VFMA_bfs_T1enc_A_txt<T: Tracer>(
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
        gs_327184: bool,
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
        // S s_0_1: call InITBlock(s_0_0)
        let s_0_1: bool = InITBlock(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b9 b1
        if s_0_1 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call HaveAArch32BF16Ext(s_1_0)
        let s_1_1: bool = HaveAArch32BF16Ext(state, tracer, s_1_0);
        // S s_1_2: not s_1_1
        let s_1_2: bool = !s_1_1;
        // N s_1_3: branch s_1_2 b8 b2
        if s_1_2 {
            return block_8(state, tracer, fn_state);
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
        // D s_2_1: read-var Vd:u8
        let s_2_1: u8 = fn_state.Vd;
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
        // N s_2_22: branch s_2_21 b7 b3
        if s_2_21 {
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
        // C s_3_0: const #0s : i
        let s_3_0: i128 = 0;
        // D s_3_1: read-var Vn:u8
        let s_3_1: u8 = fn_state.Vn;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 4u16);
        // C s_3_3: const #1u : u64
        let s_3_3: u64 = 1;
        // D s_3_4: bit-extract s_3_2 s_3_0 s_3_3
        let s_3_4: Bits = (Bits::new(
            ((s_3_2) >> (s_3_0)).value(),
            u16::try_from(s_3_3).unwrap(),
        ));
        // D s_3_5: cast reint s_3_4 -> u8
        let s_3_5: bool = ((s_3_4.value()) != 0);
        // C s_3_6: const #0s : i
        let s_3_6: i128 = 0;
        // C s_3_7: const #0u : u64
        let s_3_7: u64 = 0;
        // D s_3_8: cast zx s_3_5 -> u64
        let s_3_8: u64 = (s_3_5 as u64);
        // C s_3_9: const #1u : u64
        let s_3_9: u64 = 1;
        // D s_3_10: and s_3_8 s_3_9
        let s_3_10: u64 = ((s_3_8) & (s_3_9));
        // D s_3_11: cmp-eq s_3_10 s_3_9
        let s_3_11: bool = ((s_3_10) == (s_3_9));
        // D s_3_12: lsl s_3_8 s_3_6
        let s_3_12: u64 = s_3_8 << s_3_6;
        // D s_3_13: or s_3_7 s_3_12
        let s_3_13: u64 = ((s_3_7) | (s_3_12));
        // D s_3_14: cmpl s_3_12
        let s_3_14: u64 = !s_3_12;
        // D s_3_15: and s_3_7 s_3_14
        let s_3_15: u64 = ((s_3_7) & (s_3_14));
        // D s_3_16: select s_3_11 s_3_13 s_3_15
        let s_3_16: u64 = if s_3_11 { s_3_13 } else { s_3_15 };
        // D s_3_17: cast trunc s_3_16 -> u8
        let s_3_17: bool = ((s_3_16) != 0);
        // D s_3_18: cast zx s_3_17 -> bv
        let s_3_18: Bits = Bits::new(s_3_17 as u128, 1u16);
        // C s_3_19: const #1u : u8
        let s_3_19: bool = true;
        // C s_3_20: cast zx s_3_19 -> bv
        let s_3_20: Bits = Bits::new(s_3_19 as u128, 1u16);
        // D s_3_21: cmp-eq s_3_18 s_3_20
        let s_3_21: bool = ((s_3_18) == (s_3_20));
        // D s_3_22: write-var gs#327184 <= s_3_21
        fn_state.gs_327184 = s_3_21;
        // N s_3_23: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#327184:u8
        let s_4_0: bool = fn_state.gs_327184;
        // N s_4_1: branch s_4_0 b6 b5
        if s_4_0 {
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
        // D s_5_0: read-var D:u8
        let s_5_0: bool = fn_state.D;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // D s_5_2: read-var Vd:u8
        let s_5_2: u8 = fn_state.Vd;
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 4u16);
        // D s_5_4: cast reint s_5_1 -> u128
        let s_5_4: u128 = (s_5_1.value() as u128);
        // D s_5_5: size-of s_5_1
        let s_5_5: u16 = s_5_1.length();
        // D s_5_6: cast reint s_5_3 -> u128
        let s_5_6: u128 = (s_5_3.value() as u128);
        // D s_5_7: size-of s_5_3
        let s_5_7: u16 = s_5_3.length();
        // D s_5_8: lsl s_5_4 s_5_7
        let s_5_8: u128 = s_5_4 << s_5_7;
        // D s_5_9: or s_5_8 s_5_6
        let s_5_9: u128 = ((s_5_8) | (s_5_6));
        // D s_5_10: add s_5_5 s_5_7
        let s_5_10: u16 = (s_5_5 + s_5_7);
        // D s_5_11: create-bits s_5_9 s_5_10
        let s_5_11: Bits = Bits::new(s_5_9, s_5_10);
        // D s_5_12: cast reint s_5_11 -> u8
        let s_5_12: u8 = (s_5_11.value() as u8);
        // D s_5_13: cast zx s_5_12 -> bv
        let s_5_13: Bits = Bits::new(s_5_12 as u128, 5u16);
        // D s_5_14: cast zx s_5_13 -> i
        let s_5_14: i128 = (s_5_13.value() as i128);
        // D s_5_15: cast reint s_5_14 -> i64
        let s_5_15: i64 = (s_5_14 as i64);
        // D s_5_16: read-var N:u8
        let s_5_16: bool = fn_state.N;
        // D s_5_17: cast zx s_5_16 -> bv
        let s_5_17: Bits = Bits::new(s_5_16 as u128, 1u16);
        // D s_5_18: read-var Vn:u8
        let s_5_18: u8 = fn_state.Vn;
        // D s_5_19: cast zx s_5_18 -> bv
        let s_5_19: Bits = Bits::new(s_5_18 as u128, 4u16);
        // D s_5_20: cast reint s_5_17 -> u128
        let s_5_20: u128 = (s_5_17.value() as u128);
        // D s_5_21: size-of s_5_17
        let s_5_21: u16 = s_5_17.length();
        // D s_5_22: cast reint s_5_19 -> u128
        let s_5_22: u128 = (s_5_19.value() as u128);
        // D s_5_23: size-of s_5_19
        let s_5_23: u16 = s_5_19.length();
        // D s_5_24: lsl s_5_20 s_5_23
        let s_5_24: u128 = s_5_20 << s_5_23;
        // D s_5_25: or s_5_24 s_5_22
        let s_5_25: u128 = ((s_5_24) | (s_5_22));
        // D s_5_26: add s_5_21 s_5_23
        let s_5_26: u16 = (s_5_21 + s_5_23);
        // D s_5_27: create-bits s_5_25 s_5_26
        let s_5_27: Bits = Bits::new(s_5_25, s_5_26);
        // D s_5_28: cast reint s_5_27 -> u8
        let s_5_28: u8 = (s_5_27.value() as u8);
        // D s_5_29: cast zx s_5_28 -> bv
        let s_5_29: Bits = Bits::new(s_5_28 as u128, 5u16);
        // D s_5_30: cast zx s_5_29 -> i
        let s_5_30: i128 = (s_5_29.value() as i128);
        // D s_5_31: cast reint s_5_30 -> i64
        let s_5_31: i64 = (s_5_30 as i64);
        // C s_5_32: const #0s : i
        let s_5_32: i128 = 0;
        // D s_5_33: read-var Vm:u8
        let s_5_33: u8 = fn_state.Vm;
        // D s_5_34: cast zx s_5_33 -> bv
        let s_5_34: Bits = Bits::new(s_5_33 as u128, 4u16);
        // C s_5_35: const #1s : i64
        let s_5_35: i64 = 1;
        // C s_5_36: cast zx s_5_35 -> i
        let s_5_36: i128 = (i128::try_from(s_5_35).unwrap());
        // C s_5_37: const #2s : i
        let s_5_37: i128 = 2;
        // C s_5_38: add s_5_37 s_5_36
        let s_5_38: i128 = (s_5_37 + s_5_36);
        // D s_5_39: bit-extract s_5_34 s_5_32 s_5_38
        let s_5_39: Bits = (Bits::new(
            ((s_5_34) >> (s_5_32)).value(),
            u16::try_from(s_5_38).unwrap(),
        ));
        // D s_5_40: cast reint s_5_39 -> u8
        let s_5_40: u8 = (s_5_39.value() as u8);
        // D s_5_41: cast zx s_5_40 -> bv
        let s_5_41: Bits = Bits::new(s_5_40 as u128, 3u16);
        // D s_5_42: cast zx s_5_41 -> i
        let s_5_42: i128 = (s_5_41.value() as i128);
        // D s_5_43: cast reint s_5_42 -> i64
        let s_5_43: i64 = (s_5_42 as i64);
        // C s_5_44: const #3s : i
        let s_5_44: i128 = 3;
        // D s_5_45: read-var Vm:u8
        let s_5_45: u8 = fn_state.Vm;
        // D s_5_46: cast zx s_5_45 -> bv
        let s_5_46: Bits = Bits::new(s_5_45 as u128, 4u16);
        // C s_5_47: const #1u : u64
        let s_5_47: u64 = 1;
        // D s_5_48: bit-extract s_5_46 s_5_44 s_5_47
        let s_5_48: Bits = (Bits::new(
            ((s_5_46) >> (s_5_44)).value(),
            u16::try_from(s_5_47).unwrap(),
        ));
        // D s_5_49: cast reint s_5_48 -> u8
        let s_5_49: bool = ((s_5_48.value()) != 0);
        // C s_5_50: const #0s : i
        let s_5_50: i128 = 0;
        // C s_5_51: const #0u : u64
        let s_5_51: u64 = 0;
        // D s_5_52: cast zx s_5_49 -> u64
        let s_5_52: u64 = (s_5_49 as u64);
        // C s_5_53: const #1u : u64
        let s_5_53: u64 = 1;
        // D s_5_54: and s_5_52 s_5_53
        let s_5_54: u64 = ((s_5_52) & (s_5_53));
        // D s_5_55: cmp-eq s_5_54 s_5_53
        let s_5_55: bool = ((s_5_54) == (s_5_53));
        // D s_5_56: lsl s_5_52 s_5_50
        let s_5_56: u64 = s_5_52 << s_5_50;
        // D s_5_57: or s_5_51 s_5_56
        let s_5_57: u64 = ((s_5_51) | (s_5_56));
        // D s_5_58: cmpl s_5_56
        let s_5_58: u64 = !s_5_56;
        // D s_5_59: and s_5_51 s_5_58
        let s_5_59: u64 = ((s_5_51) & (s_5_58));
        // D s_5_60: select s_5_55 s_5_57 s_5_59
        let s_5_60: u64 = if s_5_55 { s_5_57 } else { s_5_59 };
        // D s_5_61: cast trunc s_5_60 -> u8
        let s_5_61: bool = ((s_5_60) != 0);
        // D s_5_62: read-var M:u8
        let s_5_62: bool = fn_state.M;
        // D s_5_63: cast zx s_5_62 -> bv
        let s_5_63: Bits = Bits::new(s_5_62 as u128, 1u16);
        // D s_5_64: cast zx s_5_61 -> bv
        let s_5_64: Bits = Bits::new(s_5_61 as u128, 1u16);
        // D s_5_65: cast reint s_5_63 -> u128
        let s_5_65: u128 = (s_5_63.value() as u128);
        // D s_5_66: size-of s_5_63
        let s_5_66: u16 = s_5_63.length();
        // D s_5_67: cast reint s_5_64 -> u128
        let s_5_67: u128 = (s_5_64.value() as u128);
        // D s_5_68: size-of s_5_64
        let s_5_68: u16 = s_5_64.length();
        // D s_5_69: lsl s_5_65 s_5_68
        let s_5_69: u128 = s_5_65 << s_5_68;
        // D s_5_70: or s_5_69 s_5_67
        let s_5_70: u128 = ((s_5_69) | (s_5_67));
        // D s_5_71: add s_5_66 s_5_68
        let s_5_71: u16 = (s_5_66 + s_5_68);
        // D s_5_72: create-bits s_5_70 s_5_71
        let s_5_72: Bits = Bits::new(s_5_70, s_5_71);
        // D s_5_73: cast reint s_5_72 -> u8
        let s_5_73: u8 = (s_5_72.value() as u8);
        // D s_5_74: cast zx s_5_73 -> bv
        let s_5_74: Bits = Bits::new(s_5_73 as u128, 2u16);
        // D s_5_75: cast zx s_5_74 -> i
        let s_5_75: i128 = (s_5_74.value() as i128);
        // D s_5_76: cast reint s_5_75 -> i64
        let s_5_76: i64 = (s_5_75 as i64);
        // C s_5_77: const #4s : i64
        let s_5_77: i64 = 4;
        // D s_5_78: read-var Q:u8
        let s_5_78: bool = fn_state.Q;
        // D s_5_79: cast zx s_5_78 -> bv
        let s_5_79: Bits = Bits::new(s_5_78 as u128, 1u16);
        // D s_5_80: cast zx s_5_79 -> i
        let s_5_80: i128 = (s_5_79.value() as i128);
        // D s_5_81: cast reint s_5_80 -> i64
        let s_5_81: i64 = (s_5_80 as i64);
        // D s_5_82: call execute_aarch32_instrs_VFMA_bfs_Op_A_txt(s_5_15, s_5_77, s_5_76, s_5_43, s_5_31, s_5_81)
        let s_5_82: () = execute_aarch32_instrs_VFMA_bfs_Op_A_txt(
            state,
            tracer,
            s_5_15,
            s_5_77,
            s_5_76,
            s_5_43,
            s_5_31,
            s_5_81,
        );
        // N s_5_83: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: panic
        panic!("{:?}", ());
        // N s_6_1: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #1u : u8
        let s_7_0: bool = true;
        // D s_7_1: write-var gs#327184 <= s_7_0
        fn_state.gs_327184 = s_7_0;
        // N s_7_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: panic
        panic!("{:?}", ());
        // N s_8_1: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: panic
        panic!("{:?}", ());
        // N s_9_1: return
        return;
    }
}
