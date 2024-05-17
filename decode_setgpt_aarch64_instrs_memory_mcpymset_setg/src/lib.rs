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
use execute_aarch64_instrs_memory_mcpymset_setg::*;
use ConstrainUnpredictable::*;
use EndOfInstruction::*;
use HaveFeatMOPS::*;
use HaveMTEExt::*;
use common::*;
pub fn decode_setgpt_aarch64_instrs_memory_mcpymset_setg<T: Tracer>(
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
        stageshadow_1833: u32,
        s: i64,
        stage: u32,
        gs_168274: bool,
        n: i64,
        gs_168272: bool,
        d: i64,
        gs_168270: bool,
        gs_168268: bool,
        gs_168267: bool,
        c: u32,
        options_name: u8,
        ga_265653: u8,
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
        // N s_0_3: branch s_0_2 b37 b1
        if s_0_2 {
            return block_37(state, tracer, fn_state);
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
        // S s_1_1: call HaveMTEExt(s_1_0)
        let s_1_1: bool = HaveMTEExt(state, tracer, s_1_0);
        // S s_1_2: not s_1_1
        let s_1_2: bool = !s_1_1;
        // N s_1_3: branch s_1_2 b36 b2
        if s_1_2 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var sz:u8
        let s_2_0: u8 = fn_state.sz;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #0u : u8
        let s_2_2: u8 = 0;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // D s_2_4: cmp-ne s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) != (s_2_3));
        // N s_2_5: branch s_2_4 b35 b3
        if s_2_4 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var Rd:u8
        let s_3_0: u8 = fn_state.Rd;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 5u16);
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // D s_3_4: write-var d <= s_3_3
        fn_state.d = s_3_3;
        // D s_3_5: read-var Rs:u8
        let s_3_5: u8 = fn_state.Rs;
        // D s_3_6: cast zx s_3_5 -> bv
        let s_3_6: Bits = Bits::new(s_3_5 as u128, 5u16);
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (s_3_6.value() as i128);
        // D s_3_8: cast reint s_3_7 -> i64
        let s_3_8: i64 = (s_3_7 as i64);
        // D s_3_9: write-var s <= s_3_8
        fn_state.s = s_3_8;
        // D s_3_10: read-var Rn:u8
        let s_3_10: u8 = fn_state.Rn;
        // D s_3_11: cast zx s_3_10 -> bv
        let s_3_11: Bits = Bits::new(s_3_10 as u128, 5u16);
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (s_3_11.value() as i128);
        // D s_3_13: cast reint s_3_12 -> i64
        let s_3_13: i64 = (s_3_12 as i64);
        // D s_3_14: write-var n <= s_3_13
        fn_state.n = s_3_13;
        // C s_3_15: const #0s : i
        let s_3_15: i128 = 0;
        // D s_3_16: read-var op2:u8
        let s_3_16: u8 = fn_state.op2;
        // D s_3_17: cast zx s_3_16 -> bv
        let s_3_17: Bits = Bits::new(s_3_16 as u128, 4u16);
        // C s_3_18: const #1s : i64
        let s_3_18: i64 = 1;
        // C s_3_19: cast zx s_3_18 -> i
        let s_3_19: i128 = (i128::try_from(s_3_18).unwrap());
        // C s_3_20: const #1s : i
        let s_3_20: i128 = 1;
        // C s_3_21: add s_3_20 s_3_19
        let s_3_21: i128 = (s_3_20 + s_3_19);
        // D s_3_22: bit-extract s_3_17 s_3_15 s_3_21
        let s_3_22: Bits = (Bits::new(
            ((s_3_17) >> (s_3_15)).value(),
            u16::try_from(s_3_21).unwrap(),
        ));
        // D s_3_23: cast reint s_3_22 -> u8
        let s_3_23: u8 = (s_3_22.value() as u8);
        // D s_3_24: write-var options_name <= s_3_23
        fn_state.options_name = s_3_23;
        // C s_3_25: const #2s : i
        let s_3_25: i128 = 2;
        // D s_3_26: read-var op2:u8
        let s_3_26: u8 = fn_state.op2;
        // D s_3_27: cast zx s_3_26 -> bv
        let s_3_27: Bits = Bits::new(s_3_26 as u128, 4u16);
        // C s_3_28: const #1s : i64
        let s_3_28: i64 = 1;
        // C s_3_29: cast zx s_3_28 -> i
        let s_3_29: i128 = (i128::try_from(s_3_28).unwrap());
        // C s_3_30: const #1s : i
        let s_3_30: i128 = 1;
        // C s_3_31: add s_3_30 s_3_29
        let s_3_31: i128 = (s_3_30 + s_3_29);
        // D s_3_32: bit-extract s_3_27 s_3_25 s_3_31
        let s_3_32: Bits = (Bits::new(
            ((s_3_27) >> (s_3_25)).value(),
            u16::try_from(s_3_31).unwrap(),
        ));
        // D s_3_33: cast reint s_3_32 -> u8
        let s_3_33: u8 = (s_3_32.value() as u8);
        // D s_3_34: write-var ga#265653 <= s_3_33
        fn_state.ga_265653 = s_3_33;
        // D s_3_35: read-var ga#265653:u8
        let s_3_35: u8 = fn_state.ga_265653;
        // D s_3_36: cast zx s_3_35 -> bv
        let s_3_36: Bits = Bits::new(s_3_35 as u128, 2u16);
        // C s_3_37: const #0u : u8
        let s_3_37: u8 = 0;
        // C s_3_38: cast zx s_3_37 -> bv
        let s_3_38: Bits = Bits::new(s_3_37 as u128, 2u16);
        // D s_3_39: cmp-eq s_3_36 s_3_38
        let s_3_39: bool = ((s_3_36) == (s_3_38));
        // D s_3_40: not s_3_39
        let s_3_40: bool = !s_3_39;
        // N s_3_41: branch s_3_40 b30 b4
        if s_3_40 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u32
        let s_4_0: u32 = 0;
        // D s_4_1: write-var stage <= s_4_0
        fn_state.stage = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var stage:u32
        let s_5_0: u32 = fn_state.stage;
        // D s_5_1: write-var stageshadow#1833 <= s_5_0
        fn_state.stageshadow_1833 = s_5_0;
        // C s_5_2: const #() : ()
        let s_5_2: () = ();
        // S s_5_3: call CheckMOPSEnabled(s_5_2)
        let s_5_3: () = CheckMOPSEnabled(state, tracer, s_5_2);
        // N s_5_4: jump b6
        return block_6(state, tracer, fn_state);
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
        // D s_6_2: read-var n:i64
        let s_6_2: i64 = fn_state.n;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // N s_6_5: branch s_6_4 b29 b7
        if s_6_4 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var s:i64
        let s_7_0: i64 = fn_state.s;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: read-var d:i64
        let s_7_2: i64 = fn_state.d;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // D s_7_5: write-var gs#168267 <= s_7_4
        fn_state.gs_168267 = s_7_4;
        // N s_7_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#168267:u8
        let s_8_0: bool = fn_state.gs_168267;
        // N s_8_1: branch s_8_0 b28 b9
        if s_8_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var n:i64
        let s_9_0: i64 = fn_state.n;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: read-var d:i64
        let s_9_2: i64 = fn_state.d;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
        // D s_9_5: write-var gs#168268 <= s_9_4
        fn_state.gs_168268 = s_9_4;
        // N s_9_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#168268:u8
        let s_10_0: bool = fn_state.gs_168268;
        // N s_10_1: branch s_10_0 b27 b11
        if s_10_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #31s : i
        let s_11_0: i128 = 31;
        // D s_11_1: read-var d:i64
        let s_11_1: i64 = fn_state.d;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: cmp-eq s_11_2 s_11_0
        let s_11_3: bool = ((s_11_2) == (s_11_0));
        // D s_11_4: write-var gs#168270 <= s_11_3
        fn_state.gs_168270 = s_11_3;
        // N s_11_5: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#168270:u8
        let s_12_0: bool = fn_state.gs_168270;
        // N s_12_1: branch s_12_0 b26 b13
        if s_12_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #31s : i
        let s_13_0: i128 = 31;
        // D s_13_1: read-var n:i64
        let s_13_1: i64 = fn_state.n;
        // D s_13_2: cast zx s_13_1 -> i
        let s_13_2: i128 = (i128::try_from(s_13_1).unwrap());
        // D s_13_3: cmp-eq s_13_2 s_13_0
        let s_13_3: bool = ((s_13_2) == (s_13_0));
        // D s_13_4: write-var gs#168272 <= s_13_3
        fn_state.gs_168272 = s_13_3;
        // N s_13_5: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#168272:u8
        let s_14_0: bool = fn_state.gs_168272;
        // N s_14_1: branch s_14_0 b17 b15
        if s_14_0 {
            return block_17(state, tracer, fn_state);
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
        // D s_16_0: read-var d:i64
        let s_16_0: i64 = fn_state.d;
        // D s_16_1: read-var n:i64
        let s_16_1: i64 = fn_state.n;
        // D s_16_2: read-var options_name:u8
        let s_16_2: u8 = fn_state.options_name;
        // D s_16_3: read-var s:i64
        let s_16_3: i64 = fn_state.s;
        // D s_16_4: read-var stageshadow#1833:u32
        let s_16_4: u32 = fn_state.stageshadow_1833;
        // D s_16_5: call execute_aarch64_instrs_memory_mcpymset_setg(s_16_0, s_16_1, s_16_2, s_16_3, s_16_4)
        let s_16_5: () = execute_aarch64_instrs_memory_mcpymset_setg(
            state,
            tracer,
            s_16_0,
            s_16_1,
            s_16_2,
            s_16_3,
            s_16_4,
        );
        // N s_16_6: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #76u : u32
        let s_17_0: u32 = 76;
        // S s_17_1: call ConstrainUnpredictable(s_17_0)
        let s_17_1: u32 = ConstrainUnpredictable(state, tracer, s_17_0);
        // D s_17_2: write-var c <= s_17_1
        fn_state.c = s_17_1;
        // D s_17_3: read-var c:u32
        let s_17_3: u32 = fn_state.c;
        // C s_17_4: const #2u : u32
        let s_17_4: u32 = 2;
        // D s_17_5: cmp-eq s_17_3 s_17_4
        let s_17_5: bool = ((s_17_3) == (s_17_4));
        // N s_17_6: branch s_17_5 b25 b18
        if s_17_5 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var c:u32
        let s_18_0: u32 = fn_state.c;
        // C s_18_1: const #4u : u32
        let s_18_1: u32 = 4;
        // D s_18_2: cmp-eq s_18_0 s_18_1
        let s_18_2: bool = ((s_18_0) == (s_18_1));
        // D s_18_3: write-var gs#168274 <= s_18_2
        fn_state.gs_168274 = s_18_2;
        // N s_18_4: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#168274:u8
        let s_19_0: bool = fn_state.gs_168274;
        // N s_19_1: assert s_19_0
        let s_19_1: () = assert!(s_19_0);
        // C s_19_2: const #2u : u32
        let s_19_2: u32 = 2;
        // D s_19_3: read-var c:u32
        let s_19_3: u32 = fn_state.c;
        // D s_19_4: cmp-eq s_19_2 s_19_3
        let s_19_4: bool = ((s_19_2) == (s_19_3));
        // D s_19_5: not s_19_4
        let s_19_5: bool = !s_19_4;
        // N s_19_6: branch s_19_5 b21 b20
        if s_19_5 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: panic
        panic!("{:?}", ());
        // N s_20_1: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #4u : u32
        let s_21_0: u32 = 4;
        // D s_21_1: read-var c:u32
        let s_21_1: u32 = fn_state.c;
        // D s_21_2: cmp-eq s_21_0 s_21_1
        let s_21_2: bool = ((s_21_0) == (s_21_1));
        // D s_21_3: not s_21_2
        let s_21_3: bool = !s_21_2;
        // N s_21_4: branch s_21_3 b24 b22
        if s_21_3 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call EndOfInstruction(s_22_0)
        let s_22_1: () = EndOfInstruction(state, tracer, s_22_0);
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#168274 <= s_25_0
        fn_state.gs_168274 = s_25_0;
        // N s_25_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var gs#168272 <= s_26_0
        fn_state.gs_168272 = s_26_0;
        // N s_26_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var gs#168270 <= s_27_0
        fn_state.gs_168270 = s_27_0;
        // N s_27_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#168268 <= s_28_0
        fn_state.gs_168268 = s_28_0;
        // N s_28_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var gs#168267 <= s_29_0
        fn_state.gs_168267 = s_29_0;
        // N s_29_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var ga#265653:u8
        let s_30_0: u8 = fn_state.ga_265653;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 2u16);
        // C s_30_2: const #1u : u8
        let s_30_2: u8 = 1;
        // C s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 2u16);
        // D s_30_4: cmp-eq s_30_1 s_30_3
        let s_30_4: bool = ((s_30_1) == (s_30_3));
        // D s_30_5: not s_30_4
        let s_30_5: bool = !s_30_4;
        // N s_30_6: branch s_30_5 b32 b31
        if s_30_5 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #1u : u32
        let s_31_0: u32 = 1;
        // D s_31_1: write-var stage <= s_31_0
        fn_state.stage = s_31_0;
        // N s_31_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var ga#265653:u8
        let s_32_0: u8 = fn_state.ga_265653;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 2u16);
        // C s_32_2: const #2u : u8
        let s_32_2: u8 = 2;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 2u16);
        // D s_32_4: cmp-eq s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) == (s_32_3));
        // D s_32_5: not s_32_4
        let s_32_5: bool = !s_32_4;
        // N s_32_6: branch s_32_5 b34 b33
        if s_32_5 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #2u : u32
        let s_33_0: u32 = 2;
        // D s_33_1: write-var stage <= s_33_0
        fn_state.stage = s_33_0;
        // N s_33_2: jump b5
        return block_5(state, tracer, fn_state);
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
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_36_0: panic
        panic!("{:?}", ());
        // N s_36_1: return
        return;
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_37_0: panic
        panic!("{:?}", ());
        // N s_37_1: return
        return;
    }
}
