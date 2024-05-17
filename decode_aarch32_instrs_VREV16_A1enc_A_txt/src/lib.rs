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
use ConditionPassed::*;
use execute_aarch32_instrs_VREV16_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VREV16_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    size: u8,
    Vd: u8,
    op: u8,
    Q: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        esize: i64,
        gs_317758: bool,
        d: i64,
        container_size: i64,
        ga_360054: i64,
        elements_per_container: i64,
        containers: i64,
        gs_317757: bool,
        D: bool,
        size: u8,
        Vd: u8,
        op: u8,
        Q: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        size,
        Vd,
        op,
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
        // S s_0_1: call ConditionPassed(s_0_0)
        let s_0_1: bool = ConditionPassed(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b2 b1
        if s_0_1 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var op:u8
        let s_2_0: u8 = fn_state.op;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // D s_2_4: read-var size:u8
        let s_2_4: u8 = fn_state.size;
        // D s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 2u16);
        // D s_2_6: cast zx s_2_5 -> i
        let s_2_6: i128 = (s_2_5.value() as i128);
        // D s_2_7: cast reint s_2_6 -> i64
        let s_2_7: i64 = (s_2_6 as i64);
        // D s_2_8: cast zx s_2_3 -> i
        let s_2_8: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_9: cast zx s_2_7 -> i
        let s_2_9: i128 = (i128::try_from(s_2_7).unwrap());
        // D s_2_10: add s_2_8 s_2_9
        let s_2_10: i128 = (s_2_8 + s_2_9);
        // D s_2_11: cast reint s_2_10 -> i64
        let s_2_11: i64 = (s_2_10 as i64);
        // C s_2_12: const #3s : i
        let s_2_12: i128 = 3;
        // D s_2_13: cast zx s_2_11 -> i
        let s_2_13: i128 = (i128::try_from(s_2_11).unwrap());
        // D s_2_14: cmp-ge s_2_13 s_2_12
        let s_2_14: bool = ((s_2_13) >= (s_2_12));
        // N s_2_15: branch s_2_14 b22 b3
        if s_2_14 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var Q:u8
        let s_3_0: bool = fn_state.Q;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 1u16);
        // C s_3_2: const #1u : u8
        let s_3_2: bool = true;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b18 b4
        if s_3_4 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#317758 <= s_4_0
        fn_state.gs_317758 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#317758:u8
        let s_5_0: bool = fn_state.gs_317758;
        // N s_5_1: branch s_5_0 b17 b6
        if s_5_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var size:u8
        let s_6_0: u8 = fn_state.size;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (s_6_1.value() as i128);
        // D s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // C s_6_4: const #8s : i64
        let s_6_4: i64 = 8;
        // D s_6_5: lsl s_6_4 s_6_3
        let s_6_5: i64 = s_6_4 << s_6_3;
        // D s_6_6: write-var esize <= s_6_5
        fn_state.esize = s_6_5;
        // C s_6_7: const #16s : i64
        let s_6_7: i64 = 16;
        // D s_6_8: write-var container_size <= s_6_7
        fn_state.container_size = s_6_7;
        // D s_6_9: read-var op:u8
        let s_6_9: u8 = fn_state.op;
        // D s_6_10: cast zx s_6_9 -> bv
        let s_6_10: Bits = Bits::new(s_6_9 as u128, 2u16);
        // C s_6_11: const #2u : u8
        let s_6_11: u8 = 2;
        // C s_6_12: cast zx s_6_11 -> bv
        let s_6_12: Bits = Bits::new(s_6_11 as u128, 2u16);
        // D s_6_13: cmp-eq s_6_10 s_6_12
        let s_6_13: bool = ((s_6_10) == (s_6_12));
        // D s_6_14: not s_6_13
        let s_6_14: bool = !s_6_13;
        // N s_6_15: branch s_6_14 b12 b7
        if s_6_14 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #16s : i64
        let s_7_0: i64 = 16;
        // D s_7_1: write-var container_size <= s_7_0
        fn_state.container_size = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var container_size:i64
        let s_8_0: i64 = fn_state.container_size;
        // C s_8_1: const #64s : i
        let s_8_1: i128 = 64;
        // D s_8_2: cast zx s_8_0 -> i
        let s_8_2: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_3: div s_8_1 s_8_2
        let s_8_3: i128 = ((s_8_1) / (s_8_2));
        // D s_8_4: cast reint s_8_3 -> i64
        let s_8_4: i64 = (s_8_3 as i64);
        // D s_8_5: write-var containers <= s_8_4
        fn_state.containers = s_8_4;
        // D s_8_6: cast zx s_8_0 -> i
        let s_8_6: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_7: read-var esize:i64
        let s_8_7: i64 = fn_state.esize;
        // D s_8_8: cast zx s_8_7 -> i
        let s_8_8: i128 = (i128::try_from(s_8_7).unwrap());
        // D s_8_9: div s_8_6 s_8_8
        let s_8_9: i128 = ((s_8_6) / (s_8_8));
        // D s_8_10: cast reint s_8_9 -> i64
        let s_8_10: i64 = (s_8_9 as i64);
        // D s_8_11: write-var elements_per_container <= s_8_10
        fn_state.elements_per_container = s_8_10;
        // D s_8_12: read-var D:u8
        let s_8_12: bool = fn_state.D;
        // D s_8_13: cast zx s_8_12 -> bv
        let s_8_13: Bits = Bits::new(s_8_12 as u128, 1u16);
        // D s_8_14: read-var Vd:u8
        let s_8_14: u8 = fn_state.Vd;
        // D s_8_15: cast zx s_8_14 -> bv
        let s_8_15: Bits = Bits::new(s_8_14 as u128, 4u16);
        // D s_8_16: cast reint s_8_13 -> u128
        let s_8_16: u128 = (s_8_13.value() as u128);
        // D s_8_17: size-of s_8_13
        let s_8_17: u16 = s_8_13.length();
        // D s_8_18: cast reint s_8_15 -> u128
        let s_8_18: u128 = (s_8_15.value() as u128);
        // D s_8_19: size-of s_8_15
        let s_8_19: u16 = s_8_15.length();
        // D s_8_20: lsl s_8_16 s_8_19
        let s_8_20: u128 = s_8_16 << s_8_19;
        // D s_8_21: or s_8_20 s_8_18
        let s_8_21: u128 = ((s_8_20) | (s_8_18));
        // D s_8_22: add s_8_17 s_8_19
        let s_8_22: u16 = (s_8_17 + s_8_19);
        // D s_8_23: create-bits s_8_21 s_8_22
        let s_8_23: Bits = Bits::new(s_8_21, s_8_22);
        // D s_8_24: cast reint s_8_23 -> u8
        let s_8_24: u8 = (s_8_23.value() as u8);
        // D s_8_25: cast zx s_8_24 -> bv
        let s_8_25: Bits = Bits::new(s_8_24 as u128, 5u16);
        // D s_8_26: cast zx s_8_25 -> i
        let s_8_26: i128 = (s_8_25.value() as i128);
        // D s_8_27: cast reint s_8_26 -> i64
        let s_8_27: i64 = (s_8_26 as i64);
        // D s_8_28: write-var d <= s_8_27
        fn_state.d = s_8_27;
        // D s_8_29: read-var M:u8
        let s_8_29: bool = fn_state.M;
        // D s_8_30: cast zx s_8_29 -> bv
        let s_8_30: Bits = Bits::new(s_8_29 as u128, 1u16);
        // D s_8_31: read-var Vm:u8
        let s_8_31: u8 = fn_state.Vm;
        // D s_8_32: cast zx s_8_31 -> bv
        let s_8_32: Bits = Bits::new(s_8_31 as u128, 4u16);
        // D s_8_33: cast reint s_8_30 -> u128
        let s_8_33: u128 = (s_8_30.value() as u128);
        // D s_8_34: size-of s_8_30
        let s_8_34: u16 = s_8_30.length();
        // D s_8_35: cast reint s_8_32 -> u128
        let s_8_35: u128 = (s_8_32.value() as u128);
        // D s_8_36: size-of s_8_32
        let s_8_36: u16 = s_8_32.length();
        // D s_8_37: lsl s_8_33 s_8_36
        let s_8_37: u128 = s_8_33 << s_8_36;
        // D s_8_38: or s_8_37 s_8_35
        let s_8_38: u128 = ((s_8_37) | (s_8_35));
        // D s_8_39: add s_8_34 s_8_36
        let s_8_39: u16 = (s_8_34 + s_8_36);
        // D s_8_40: create-bits s_8_38 s_8_39
        let s_8_40: Bits = Bits::new(s_8_38, s_8_39);
        // D s_8_41: cast reint s_8_40 -> u8
        let s_8_41: u8 = (s_8_40.value() as u8);
        // D s_8_42: cast zx s_8_41 -> bv
        let s_8_42: Bits = Bits::new(s_8_41 as u128, 5u16);
        // D s_8_43: cast zx s_8_42 -> i
        let s_8_43: i128 = (s_8_42.value() as i128);
        // D s_8_44: cast reint s_8_43 -> i64
        let s_8_44: i64 = (s_8_43 as i64);
        // D s_8_45: write-var m <= s_8_44
        fn_state.m = s_8_44;
        // D s_8_46: read-var Q:u8
        let s_8_46: bool = fn_state.Q;
        // D s_8_47: cast zx s_8_46 -> bv
        let s_8_47: Bits = Bits::new(s_8_46 as u128, 1u16);
        // C s_8_48: const #0u : u8
        let s_8_48: bool = false;
        // C s_8_49: cast zx s_8_48 -> bv
        let s_8_49: Bits = Bits::new(s_8_48 as u128, 1u16);
        // D s_8_50: cmp-eq s_8_47 s_8_49
        let s_8_50: bool = ((s_8_47) == (s_8_49));
        // N s_8_51: branch s_8_50 b11 b9
        if s_8_50 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #2s : i64
        let s_9_0: i64 = 2;
        // D s_9_1: write-var ga#360054 <= s_9_0
        fn_state.ga_360054 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var ga#360054:i64
        let s_10_0: i64 = fn_state.ga_360054;
        // D s_10_1: read-var esize:i64
        let s_10_1: i64 = fn_state.esize;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: cast reint s_10_2 -> i64
        let s_10_3: i64 = (s_10_2 as i64);
        // D s_10_4: read-var containers:i64
        let s_10_4: i64 = fn_state.containers;
        // D s_10_5: cast zx s_10_4 -> i
        let s_10_5: i128 = (i128::try_from(s_10_4).unwrap());
        // D s_10_6: read-var elements_per_container:i64
        let s_10_6: i64 = fn_state.elements_per_container;
        // D s_10_7: cast zx s_10_6 -> i
        let s_10_7: i128 = (i128::try_from(s_10_6).unwrap());
        // D s_10_8: read-var d:i64
        let s_10_8: i64 = fn_state.d;
        // D s_10_9: read-var m:i64
        let s_10_9: i64 = fn_state.m;
        // D s_10_10: call execute_aarch32_instrs_VREV16_Op_A_txt(s_10_5, s_10_8, s_10_7, s_10_3, s_10_9, s_10_0)
        let s_10_10: () = execute_aarch32_instrs_VREV16_Op_A_txt(
            state,
            tracer,
            s_10_5,
            s_10_8,
            s_10_7,
            s_10_3,
            s_10_9,
            s_10_0,
        );
        // N s_10_11: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #1s : i64
        let s_11_0: i64 = 1;
        // D s_11_1: write-var ga#360054 <= s_11_0
        fn_state.ga_360054 = s_11_0;
        // N s_11_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var op:u8
        let s_12_0: u8 = fn_state.op;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 2u16);
        // C s_12_2: const #1u : u8
        let s_12_2: u8 = 1;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 2u16);
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // D s_12_5: not s_12_4
        let s_12_5: bool = !s_12_4;
        // N s_12_6: branch s_12_5 b14 b13
        if s_12_5 {
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
        // C s_13_0: const #32s : i64
        let s_13_0: i64 = 32;
        // D s_13_1: write-var container_size <= s_13_0
        fn_state.container_size = s_13_0;
        // N s_13_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var op:u8
        let s_14_0: u8 = fn_state.op;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 2u16);
        // C s_14_2: const #0u : u8
        let s_14_2: u8 = 0;
        // C s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 2u16);
        // D s_14_4: cmp-eq s_14_1 s_14_3
        let s_14_4: bool = ((s_14_1) == (s_14_3));
        // D s_14_5: not s_14_4
        let s_14_5: bool = !s_14_4;
        // N s_14_6: branch s_14_5 b16 b15
        if s_14_5 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #64s : i64
        let s_15_0: i64 = 64;
        // D s_15_1: write-var container_size <= s_15_0
        fn_state.container_size = s_15_0;
        // N s_15_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: panic
        panic!("{:?}", ());
        // N s_17_1: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0s : i
        let s_18_0: i128 = 0;
        // D s_18_1: read-var Vd:u8
        let s_18_1: u8 = fn_state.Vd;
        // D s_18_2: cast zx s_18_1 -> bv
        let s_18_2: Bits = Bits::new(s_18_1 as u128, 4u16);
        // C s_18_3: const #1u : u64
        let s_18_3: u64 = 1;
        // D s_18_4: bit-extract s_18_2 s_18_0 s_18_3
        let s_18_4: Bits = (Bits::new(
            ((s_18_2) >> (s_18_0)).value(),
            u16::try_from(s_18_3).unwrap(),
        ));
        // D s_18_5: cast reint s_18_4 -> u8
        let s_18_5: bool = ((s_18_4.value()) != 0);
        // C s_18_6: const #0s : i
        let s_18_6: i128 = 0;
        // C s_18_7: const #0u : u64
        let s_18_7: u64 = 0;
        // D s_18_8: cast zx s_18_5 -> u64
        let s_18_8: u64 = (s_18_5 as u64);
        // C s_18_9: const #1u : u64
        let s_18_9: u64 = 1;
        // D s_18_10: and s_18_8 s_18_9
        let s_18_10: u64 = ((s_18_8) & (s_18_9));
        // D s_18_11: cmp-eq s_18_10 s_18_9
        let s_18_11: bool = ((s_18_10) == (s_18_9));
        // D s_18_12: lsl s_18_8 s_18_6
        let s_18_12: u64 = s_18_8 << s_18_6;
        // D s_18_13: or s_18_7 s_18_12
        let s_18_13: u64 = ((s_18_7) | (s_18_12));
        // D s_18_14: cmpl s_18_12
        let s_18_14: u64 = !s_18_12;
        // D s_18_15: and s_18_7 s_18_14
        let s_18_15: u64 = ((s_18_7) & (s_18_14));
        // D s_18_16: select s_18_11 s_18_13 s_18_15
        let s_18_16: u64 = if s_18_11 { s_18_13 } else { s_18_15 };
        // D s_18_17: cast trunc s_18_16 -> u8
        let s_18_17: bool = ((s_18_16) != 0);
        // D s_18_18: cast zx s_18_17 -> bv
        let s_18_18: Bits = Bits::new(s_18_17 as u128, 1u16);
        // C s_18_19: const #1u : u8
        let s_18_19: bool = true;
        // C s_18_20: cast zx s_18_19 -> bv
        let s_18_20: Bits = Bits::new(s_18_19 as u128, 1u16);
        // D s_18_21: cmp-eq s_18_18 s_18_20
        let s_18_21: bool = ((s_18_18) == (s_18_20));
        // N s_18_22: branch s_18_21 b21 b19
        if s_18_21 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0s : i
        let s_19_0: i128 = 0;
        // D s_19_1: read-var Vm:u8
        let s_19_1: u8 = fn_state.Vm;
        // D s_19_2: cast zx s_19_1 -> bv
        let s_19_2: Bits = Bits::new(s_19_1 as u128, 4u16);
        // C s_19_3: const #1u : u64
        let s_19_3: u64 = 1;
        // D s_19_4: bit-extract s_19_2 s_19_0 s_19_3
        let s_19_4: Bits = (Bits::new(
            ((s_19_2) >> (s_19_0)).value(),
            u16::try_from(s_19_3).unwrap(),
        ));
        // D s_19_5: cast reint s_19_4 -> u8
        let s_19_5: bool = ((s_19_4.value()) != 0);
        // C s_19_6: const #0s : i
        let s_19_6: i128 = 0;
        // C s_19_7: const #0u : u64
        let s_19_7: u64 = 0;
        // D s_19_8: cast zx s_19_5 -> u64
        let s_19_8: u64 = (s_19_5 as u64);
        // C s_19_9: const #1u : u64
        let s_19_9: u64 = 1;
        // D s_19_10: and s_19_8 s_19_9
        let s_19_10: u64 = ((s_19_8) & (s_19_9));
        // D s_19_11: cmp-eq s_19_10 s_19_9
        let s_19_11: bool = ((s_19_10) == (s_19_9));
        // D s_19_12: lsl s_19_8 s_19_6
        let s_19_12: u64 = s_19_8 << s_19_6;
        // D s_19_13: or s_19_7 s_19_12
        let s_19_13: u64 = ((s_19_7) | (s_19_12));
        // D s_19_14: cmpl s_19_12
        let s_19_14: u64 = !s_19_12;
        // D s_19_15: and s_19_7 s_19_14
        let s_19_15: u64 = ((s_19_7) & (s_19_14));
        // D s_19_16: select s_19_11 s_19_13 s_19_15
        let s_19_16: u64 = if s_19_11 { s_19_13 } else { s_19_15 };
        // D s_19_17: cast trunc s_19_16 -> u8
        let s_19_17: bool = ((s_19_16) != 0);
        // D s_19_18: cast zx s_19_17 -> bv
        let s_19_18: Bits = Bits::new(s_19_17 as u128, 1u16);
        // C s_19_19: const #1u : u8
        let s_19_19: bool = true;
        // C s_19_20: cast zx s_19_19 -> bv
        let s_19_20: Bits = Bits::new(s_19_19 as u128, 1u16);
        // D s_19_21: cmp-eq s_19_18 s_19_20
        let s_19_21: bool = ((s_19_18) == (s_19_20));
        // D s_19_22: write-var gs#317757 <= s_19_21
        fn_state.gs_317757 = s_19_21;
        // N s_19_23: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#317757:u8
        let s_20_0: bool = fn_state.gs_317757;
        // D s_20_1: write-var gs#317758 <= s_20_0
        fn_state.gs_317758 = s_20_0;
        // N s_20_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#317757 <= s_21_0
        fn_state.gs_317757 = s_21_0;
        // N s_21_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: panic
        panic!("{:?}", ());
        // N s_22_1: return
        return;
    }
}
