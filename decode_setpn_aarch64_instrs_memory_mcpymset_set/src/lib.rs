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
use CheckMOPSEnabled::*;
use ConstrainUnpredictable::*;
use execute_aarch64_instrs_memory_mcpymset_set::*;
use HaveFeatMOPS::*;
use EndOfInstruction::*;
use common::*;
pub fn decode_setpn_aarch64_instrs_memory_mcpymset_set<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    op2: u8,
    Rs: u8,
    sz: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_167949: bool,
        s: i64,
        gs_167945: bool,
        stageshadow_1826: u32,
        stage: u32,
        n: i64,
        gs_167944: bool,
        d: i64,
        gs_167951: bool,
        ga_265430: u8,
        c: u32,
        options_name: u8,
        gs_167947: bool,
        Rd: u8,
        Rn: u8,
        op2: u8,
        Rs: u8,
        sz: u8,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        op2,
        Rs,
        sz,
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
        // S s_0_1: call HaveFeatMOPS(s_0_0)
        let s_0_1: bool = HaveFeatMOPS(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b35 b1
        if s_0_2 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var sz:u8
        let s_1_0: u8 = fn_state.sz;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 2u16);
        // C s_1_2: const #0u : u8
        let s_1_2: u8 = 0;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 2u16);
        // D s_1_4: cmp-ne s_1_1 s_1_3
        let s_1_4: bool = ((s_1_1) != (s_1_3));
        // N s_1_5: branch s_1_4 b34 b2
        if s_1_4 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var Rd:u8
        let s_2_0: u8 = fn_state.Rd;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 5u16);
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // D s_2_4: write-var d <= s_2_3
        fn_state.d = s_2_3;
        // D s_2_5: read-var Rs:u8
        let s_2_5: u8 = fn_state.Rs;
        // D s_2_6: cast zx s_2_5 -> bv
        let s_2_6: Bits = Bits::new(s_2_5 as u128, 5u16);
        // D s_2_7: cast zx s_2_6 -> i
        let s_2_7: i128 = (s_2_6.value() as i128);
        // D s_2_8: cast reint s_2_7 -> i64
        let s_2_8: i64 = (s_2_7 as i64);
        // D s_2_9: write-var s <= s_2_8
        fn_state.s = s_2_8;
        // D s_2_10: read-var Rn:u8
        let s_2_10: u8 = fn_state.Rn;
        // D s_2_11: cast zx s_2_10 -> bv
        let s_2_11: Bits = Bits::new(s_2_10 as u128, 5u16);
        // D s_2_12: cast zx s_2_11 -> i
        let s_2_12: i128 = (s_2_11.value() as i128);
        // D s_2_13: cast reint s_2_12 -> i64
        let s_2_13: i64 = (s_2_12 as i64);
        // D s_2_14: write-var n <= s_2_13
        fn_state.n = s_2_13;
        // C s_2_15: const #0s : i
        let s_2_15: i128 = 0;
        // D s_2_16: read-var op2:u8
        let s_2_16: u8 = fn_state.op2;
        // D s_2_17: cast zx s_2_16 -> bv
        let s_2_17: Bits = Bits::new(s_2_16 as u128, 4u16);
        // C s_2_18: const #1s : i64
        let s_2_18: i64 = 1;
        // C s_2_19: cast zx s_2_18 -> i
        let s_2_19: i128 = (i128::try_from(s_2_18).unwrap());
        // C s_2_20: const #1s : i
        let s_2_20: i128 = 1;
        // C s_2_21: add s_2_20 s_2_19
        let s_2_21: i128 = (s_2_20 + s_2_19);
        // D s_2_22: bit-extract s_2_17 s_2_15 s_2_21
        let s_2_22: Bits = (Bits::new(
            ((s_2_17) >> (s_2_15)).value(),
            u16::try_from(s_2_21).unwrap(),
        ));
        // D s_2_23: cast reint s_2_22 -> u8
        let s_2_23: u8 = (s_2_22.value() as u8);
        // D s_2_24: write-var options_name <= s_2_23
        fn_state.options_name = s_2_23;
        // C s_2_25: const #2s : i
        let s_2_25: i128 = 2;
        // D s_2_26: read-var op2:u8
        let s_2_26: u8 = fn_state.op2;
        // D s_2_27: cast zx s_2_26 -> bv
        let s_2_27: Bits = Bits::new(s_2_26 as u128, 4u16);
        // C s_2_28: const #1s : i64
        let s_2_28: i64 = 1;
        // C s_2_29: cast zx s_2_28 -> i
        let s_2_29: i128 = (i128::try_from(s_2_28).unwrap());
        // C s_2_30: const #1s : i
        let s_2_30: i128 = 1;
        // C s_2_31: add s_2_30 s_2_29
        let s_2_31: i128 = (s_2_30 + s_2_29);
        // D s_2_32: bit-extract s_2_27 s_2_25 s_2_31
        let s_2_32: Bits = (Bits::new(
            ((s_2_27) >> (s_2_25)).value(),
            u16::try_from(s_2_31).unwrap(),
        ));
        // D s_2_33: cast reint s_2_32 -> u8
        let s_2_33: u8 = (s_2_32.value() as u8);
        // D s_2_34: write-var ga#265430 <= s_2_33
        fn_state.ga_265430 = s_2_33;
        // D s_2_35: read-var ga#265430:u8
        let s_2_35: u8 = fn_state.ga_265430;
        // D s_2_36: cast zx s_2_35 -> bv
        let s_2_36: Bits = Bits::new(s_2_35 as u128, 2u16);
        // C s_2_37: const #0u : u8
        let s_2_37: u8 = 0;
        // C s_2_38: cast zx s_2_37 -> bv
        let s_2_38: Bits = Bits::new(s_2_37 as u128, 2u16);
        // D s_2_39: cmp-eq s_2_36 s_2_38
        let s_2_39: bool = ((s_2_36) == (s_2_38));
        // D s_2_40: not s_2_39
        let s_2_40: bool = !s_2_39;
        // N s_2_41: branch s_2_40 b29 b3
        if s_2_40 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u32
        let s_3_0: u32 = 0;
        // D s_3_1: write-var stage <= s_3_0
        fn_state.stage = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var stage:u32
        let s_4_0: u32 = fn_state.stage;
        // D s_4_1: write-var stageshadow#1826 <= s_4_0
        fn_state.stageshadow_1826 = s_4_0;
        // C s_4_2: const #() : ()
        let s_4_2: () = ();
        // S s_4_3: call CheckMOPSEnabled(s_4_2)
        let s_4_3: () = CheckMOPSEnabled(state, tracer, s_4_2);
        // N s_4_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var s:i64
        let s_5_0: i64 = fn_state.s;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var n:i64
        let s_5_2: i64 = fn_state.n;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b28 b6
        if s_5_4 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var s:i64
        let s_6_0: i64 = fn_state.s;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: read-var d:i64
        let s_6_2: i64 = fn_state.d;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // D s_6_5: write-var gs#167944 <= s_6_4
        fn_state.gs_167944 = s_6_4;
        // N s_6_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#167944:u8
        let s_7_0: bool = fn_state.gs_167944;
        // N s_7_1: branch s_7_0 b27 b8
        if s_7_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var n:i64
        let s_8_0: i64 = fn_state.n;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: read-var d:i64
        let s_8_2: i64 = fn_state.d;
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // D s_8_5: write-var gs#167945 <= s_8_4
        fn_state.gs_167945 = s_8_4;
        // N s_8_6: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#167945:u8
        let s_9_0: bool = fn_state.gs_167945;
        // N s_9_1: branch s_9_0 b26 b10
        if s_9_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #31s : i
        let s_10_0: i128 = 31;
        // D s_10_1: read-var d:i64
        let s_10_1: i64 = fn_state.d;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: cmp-eq s_10_2 s_10_0
        let s_10_3: bool = ((s_10_2) == (s_10_0));
        // D s_10_4: write-var gs#167947 <= s_10_3
        fn_state.gs_167947 = s_10_3;
        // N s_10_5: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#167947:u8
        let s_11_0: bool = fn_state.gs_167947;
        // N s_11_1: branch s_11_0 b25 b12
        if s_11_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #31s : i
        let s_12_0: i128 = 31;
        // D s_12_1: read-var n:i64
        let s_12_1: i64 = fn_state.n;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: cmp-eq s_12_2 s_12_0
        let s_12_3: bool = ((s_12_2) == (s_12_0));
        // D s_12_4: write-var gs#167949 <= s_12_3
        fn_state.gs_167949 = s_12_3;
        // N s_12_5: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#167949:u8
        let s_13_0: bool = fn_state.gs_167949;
        // N s_13_1: branch s_13_0 b16 b14
        if s_13_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var d:i64
        let s_15_0: i64 = fn_state.d;
        // D s_15_1: read-var n:i64
        let s_15_1: i64 = fn_state.n;
        // D s_15_2: read-var options_name:u8
        let s_15_2: u8 = fn_state.options_name;
        // D s_15_3: read-var s:i64
        let s_15_3: i64 = fn_state.s;
        // D s_15_4: read-var stageshadow#1826:u32
        let s_15_4: u32 = fn_state.stageshadow_1826;
        // D s_15_5: call execute_aarch64_instrs_memory_mcpymset_set(s_15_0, s_15_1, s_15_2, s_15_3, s_15_4)
        let s_15_5: () = execute_aarch64_instrs_memory_mcpymset_set(
            state,
            tracer,
            s_15_0,
            s_15_1,
            s_15_2,
            s_15_3,
            s_15_4,
        );
        // N s_15_6: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #76u : u32
        let s_16_0: u32 = 76;
        // S s_16_1: call ConstrainUnpredictable(s_16_0)
        let s_16_1: u32 = ConstrainUnpredictable(state, tracer, s_16_0);
        // D s_16_2: write-var c <= s_16_1
        fn_state.c = s_16_1;
        // D s_16_3: read-var c:u32
        let s_16_3: u32 = fn_state.c;
        // C s_16_4: const #2u : u32
        let s_16_4: u32 = 2;
        // D s_16_5: cmp-eq s_16_3 s_16_4
        let s_16_5: bool = ((s_16_3) == (s_16_4));
        // N s_16_6: branch s_16_5 b24 b17
        if s_16_5 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var c:u32
        let s_17_0: u32 = fn_state.c;
        // C s_17_1: const #4u : u32
        let s_17_1: u32 = 4;
        // D s_17_2: cmp-eq s_17_0 s_17_1
        let s_17_2: bool = ((s_17_0) == (s_17_1));
        // D s_17_3: write-var gs#167951 <= s_17_2
        fn_state.gs_167951 = s_17_2;
        // N s_17_4: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#167951:u8
        let s_18_0: bool = fn_state.gs_167951;
        // N s_18_1: assert s_18_0
        let s_18_1: () = assert!(s_18_0);
        // C s_18_2: const #2u : u32
        let s_18_2: u32 = 2;
        // D s_18_3: read-var c:u32
        let s_18_3: u32 = fn_state.c;
        // D s_18_4: cmp-eq s_18_2 s_18_3
        let s_18_4: bool = ((s_18_2) == (s_18_3));
        // D s_18_5: not s_18_4
        let s_18_5: bool = !s_18_4;
        // N s_18_6: branch s_18_5 b20 b19
        if s_18_5 {
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
        // N s_19_0: panic
        panic!("{:?}", ());
        // N s_19_1: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #4u : u32
        let s_20_0: u32 = 4;
        // D s_20_1: read-var c:u32
        let s_20_1: u32 = fn_state.c;
        // D s_20_2: cmp-eq s_20_0 s_20_1
        let s_20_2: bool = ((s_20_0) == (s_20_1));
        // D s_20_3: not s_20_2
        let s_20_3: bool = !s_20_2;
        // N s_20_4: branch s_20_3 b23 b21
        if s_20_3 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call EndOfInstruction(s_21_0)
        let s_21_1: () = EndOfInstruction(state, tracer, s_21_0);
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#167951 <= s_24_0
        fn_state.gs_167951 = s_24_0;
        // N s_24_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#167949 <= s_25_0
        fn_state.gs_167949 = s_25_0;
        // N s_25_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var gs#167947 <= s_26_0
        fn_state.gs_167947 = s_26_0;
        // N s_26_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var gs#167945 <= s_27_0
        fn_state.gs_167945 = s_27_0;
        // N s_27_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#167944 <= s_28_0
        fn_state.gs_167944 = s_28_0;
        // N s_28_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var ga#265430:u8
        let s_29_0: u8 = fn_state.ga_265430;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 2u16);
        // C s_29_2: const #1u : u8
        let s_29_2: u8 = 1;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 2u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: not s_29_4
        let s_29_5: bool = !s_29_4;
        // N s_29_6: branch s_29_5 b31 b30
        if s_29_5 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #1u : u32
        let s_30_0: u32 = 1;
        // D s_30_1: write-var stage <= s_30_0
        fn_state.stage = s_30_0;
        // N s_30_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var ga#265430:u8
        let s_31_0: u8 = fn_state.ga_265430;
        // D s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 2u16);
        // C s_31_2: const #2u : u8
        let s_31_2: u8 = 2;
        // C s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 2u16);
        // D s_31_4: cmp-eq s_31_1 s_31_3
        let s_31_4: bool = ((s_31_1) == (s_31_3));
        // D s_31_5: not s_31_4
        let s_31_5: bool = !s_31_4;
        // N s_31_6: branch s_31_5 b33 b32
        if s_31_5 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #2u : u32
        let s_32_0: u32 = 2;
        // D s_32_1: write-var stage <= s_32_0
        fn_state.stage = s_32_0;
        // N s_32_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_33_0: panic
        panic!("{:?}", ());
        // N s_33_1: return
        return;
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_34_0: panic
        panic!("{:?}", ());
        // N s_34_1: return
        return;
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_35_0: panic
        panic!("{:?}", ());
        // N s_35_1: return
        return;
    }
}
