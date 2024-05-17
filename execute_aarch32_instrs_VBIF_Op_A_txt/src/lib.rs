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
use D_set::*;
use CheckAdvSIMDEnabled::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VBIF_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    n: i64,
    operation: u32,
    regs: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        gs_306630: i64,
        d: i64,
        m: i64,
        n: i64,
        operation: u32,
        regs: i64,
    }
    let fn_state = FunctionState {
        d,
        m,
        n,
        operation,
        regs,
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
        // S s_0_1: call CheckAdvSIMDEnabled(s_0_0)
        let s_0_1: () = CheckAdvSIMDEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0s : i64
        let s_1_0: i64 = 0;
        // C s_1_1: const #1s : i
        let s_1_1: i128 = 1;
        // D s_1_2: read-var regs:i64
        let s_1_2: i64 = fn_state.regs;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: sub s_1_3 s_1_1
        let s_1_4: i128 = ((s_1_3) - (s_1_1));
        // D s_1_5: cast reint s_1_4 -> i64
        let s_1_5: i64 = (s_1_4 as i64);
        // D s_1_6: write-var gs#306630 <= s_1_5
        fn_state.gs_306630 = s_1_5;
        // D s_1_7: write-var r <= s_1_0
        fn_state.r = s_1_0;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // D s_2_1: read-var gs#306630:i64
        let s_2_1: i64 = fn_state.gs_306630;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b11 b3
        if s_2_2 {
            return block_11(state, tracer, fn_state);
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
        // D s_3_1: read-var operation:u32
        let s_3_1: u32 = fn_state.operation;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // D s_3_3: not s_3_2
        let s_3_3: bool = !s_3_2;
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
        // D s_4_0: read-var d:i64
        let s_4_0: i64 = fn_state.d;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: read-var r:i64
        let s_4_2: i64 = fn_state.r;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: add s_4_1 s_4_3
        let s_4_4: i128 = (s_4_1 + s_4_3);
        // D s_4_5: cast reint s_4_4 -> i64
        let s_4_5: i64 = (s_4_4 as i64);
        // D s_4_6: read-var d:i64
        let s_4_6: i64 = fn_state.d;
        // D s_4_7: cast zx s_4_6 -> i
        let s_4_7: i128 = (i128::try_from(s_4_6).unwrap());
        // D s_4_8: read-var r:i64
        let s_4_8: i64 = fn_state.r;
        // D s_4_9: cast zx s_4_8 -> i
        let s_4_9: i128 = (i128::try_from(s_4_8).unwrap());
        // D s_4_10: add s_4_7 s_4_9
        let s_4_10: i128 = (s_4_7 + s_4_9);
        // D s_4_11: cast reint s_4_10 -> i64
        let s_4_11: i64 = (s_4_10 as i64);
        // D s_4_12: cast zx s_4_11 -> i
        let s_4_12: i128 = (i128::try_from(s_4_11).unwrap());
        // D s_4_13: call D_read(s_4_12)
        let s_4_13: u64 = D_read(state, tracer, s_4_12);
        // D s_4_14: read-var m:i64
        let s_4_14: i64 = fn_state.m;
        // D s_4_15: cast zx s_4_14 -> i
        let s_4_15: i128 = (i128::try_from(s_4_14).unwrap());
        // D s_4_16: read-var r:i64
        let s_4_16: i64 = fn_state.r;
        // D s_4_17: cast zx s_4_16 -> i
        let s_4_17: i128 = (i128::try_from(s_4_16).unwrap());
        // D s_4_18: add s_4_15 s_4_17
        let s_4_18: i128 = (s_4_15 + s_4_17);
        // D s_4_19: cast reint s_4_18 -> i64
        let s_4_19: i64 = (s_4_18 as i64);
        // D s_4_20: cast zx s_4_19 -> i
        let s_4_20: i128 = (i128::try_from(s_4_19).unwrap());
        // D s_4_21: call D_read(s_4_20)
        let s_4_21: u64 = D_read(state, tracer, s_4_20);
        // D s_4_22: cast zx s_4_13 -> bv
        let s_4_22: Bits = Bits::new(s_4_13 as u128, 64u16);
        // D s_4_23: cast zx s_4_21 -> bv
        let s_4_23: Bits = Bits::new(s_4_21 as u128, 64u16);
        // D s_4_24: and s_4_22 s_4_23
        let s_4_24: Bits = ((s_4_22) & (s_4_23));
        // D s_4_25: cast reint s_4_24 -> u64
        let s_4_25: u64 = (s_4_24.value() as u64);
        // D s_4_26: read-var n:i64
        let s_4_26: i64 = fn_state.n;
        // D s_4_27: cast zx s_4_26 -> i
        let s_4_27: i128 = (i128::try_from(s_4_26).unwrap());
        // D s_4_28: read-var r:i64
        let s_4_28: i64 = fn_state.r;
        // D s_4_29: cast zx s_4_28 -> i
        let s_4_29: i128 = (i128::try_from(s_4_28).unwrap());
        // D s_4_30: add s_4_27 s_4_29
        let s_4_30: i128 = (s_4_27 + s_4_29);
        // D s_4_31: cast reint s_4_30 -> i64
        let s_4_31: i64 = (s_4_30 as i64);
        // D s_4_32: cast zx s_4_31 -> i
        let s_4_32: i128 = (i128::try_from(s_4_31).unwrap());
        // D s_4_33: call D_read(s_4_32)
        let s_4_33: u64 = D_read(state, tracer, s_4_32);
        // D s_4_34: read-var m:i64
        let s_4_34: i64 = fn_state.m;
        // D s_4_35: cast zx s_4_34 -> i
        let s_4_35: i128 = (i128::try_from(s_4_34).unwrap());
        // D s_4_36: read-var r:i64
        let s_4_36: i64 = fn_state.r;
        // D s_4_37: cast zx s_4_36 -> i
        let s_4_37: i128 = (i128::try_from(s_4_36).unwrap());
        // D s_4_38: add s_4_35 s_4_37
        let s_4_38: i128 = (s_4_35 + s_4_37);
        // D s_4_39: cast reint s_4_38 -> i64
        let s_4_39: i64 = (s_4_38 as i64);
        // D s_4_40: cast zx s_4_39 -> i
        let s_4_40: i128 = (i128::try_from(s_4_39).unwrap());
        // D s_4_41: call D_read(s_4_40)
        let s_4_41: u64 = D_read(state, tracer, s_4_40);
        // D s_4_42: cast zx s_4_41 -> bv
        let s_4_42: Bits = Bits::new(s_4_41 as u128, 64u16);
        // D s_4_43: not s_4_42
        let s_4_43: Bits = !s_4_42;
        // D s_4_44: cast reint s_4_43 -> u64
        let s_4_44: u64 = (s_4_43.value() as u64);
        // D s_4_45: cast zx s_4_33 -> bv
        let s_4_45: Bits = Bits::new(s_4_33 as u128, 64u16);
        // D s_4_46: cast zx s_4_44 -> bv
        let s_4_46: Bits = Bits::new(s_4_44 as u128, 64u16);
        // D s_4_47: and s_4_45 s_4_46
        let s_4_47: Bits = ((s_4_45) & (s_4_46));
        // D s_4_48: cast reint s_4_47 -> u64
        let s_4_48: u64 = (s_4_47.value() as u64);
        // D s_4_49: cast zx s_4_25 -> bv
        let s_4_49: Bits = Bits::new(s_4_25 as u128, 64u16);
        // D s_4_50: cast zx s_4_48 -> bv
        let s_4_50: Bits = Bits::new(s_4_48 as u128, 64u16);
        // D s_4_51: or s_4_49 s_4_50
        let s_4_51: Bits = ((s_4_49) | (s_4_50));
        // D s_4_52: cast reint s_4_51 -> u64
        let s_4_52: u64 = (s_4_51.value() as u64);
        // D s_4_53: cast zx s_4_5 -> i
        let s_4_53: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_54: call D_set(s_4_53, s_4_52)
        let s_4_54: () = D_set(state, tracer, s_4_53, s_4_52);
        // N s_4_55: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var r:i64
        let s_5_0: i64 = fn_state.r;
        // C s_5_1: const #1s : i64
        let s_5_1: i64 = 1;
        // D s_5_2: add s_5_0 s_5_1
        let s_5_2: i64 = (s_5_0 + s_5_1);
        // D s_5_3: write-var r <= s_5_2
        fn_state.r = s_5_2;
        // N s_5_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #1u : u32
        let s_6_0: u32 = 1;
        // D s_6_1: read-var operation:u32
        let s_6_1: u32 = fn_state.operation;
        // D s_6_2: cmp-eq s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) == (s_6_1));
        // D s_6_3: not s_6_2
        let s_6_3: bool = !s_6_2;
        // N s_6_4: branch s_6_3 b8 b7
        if s_6_3 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var d:i64
        let s_7_0: i64 = fn_state.d;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: read-var r:i64
        let s_7_2: i64 = fn_state.r;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: add s_7_1 s_7_3
        let s_7_4: i128 = (s_7_1 + s_7_3);
        // D s_7_5: cast reint s_7_4 -> i64
        let s_7_5: i64 = (s_7_4 as i64);
        // D s_7_6: read-var n:i64
        let s_7_6: i64 = fn_state.n;
        // D s_7_7: cast zx s_7_6 -> i
        let s_7_7: i128 = (i128::try_from(s_7_6).unwrap());
        // D s_7_8: read-var r:i64
        let s_7_8: i64 = fn_state.r;
        // D s_7_9: cast zx s_7_8 -> i
        let s_7_9: i128 = (i128::try_from(s_7_8).unwrap());
        // D s_7_10: add s_7_7 s_7_9
        let s_7_10: i128 = (s_7_7 + s_7_9);
        // D s_7_11: cast reint s_7_10 -> i64
        let s_7_11: i64 = (s_7_10 as i64);
        // D s_7_12: cast zx s_7_11 -> i
        let s_7_12: i128 = (i128::try_from(s_7_11).unwrap());
        // D s_7_13: call D_read(s_7_12)
        let s_7_13: u64 = D_read(state, tracer, s_7_12);
        // D s_7_14: read-var m:i64
        let s_7_14: i64 = fn_state.m;
        // D s_7_15: cast zx s_7_14 -> i
        let s_7_15: i128 = (i128::try_from(s_7_14).unwrap());
        // D s_7_16: read-var r:i64
        let s_7_16: i64 = fn_state.r;
        // D s_7_17: cast zx s_7_16 -> i
        let s_7_17: i128 = (i128::try_from(s_7_16).unwrap());
        // D s_7_18: add s_7_15 s_7_17
        let s_7_18: i128 = (s_7_15 + s_7_17);
        // D s_7_19: cast reint s_7_18 -> i64
        let s_7_19: i64 = (s_7_18 as i64);
        // D s_7_20: cast zx s_7_19 -> i
        let s_7_20: i128 = (i128::try_from(s_7_19).unwrap());
        // D s_7_21: call D_read(s_7_20)
        let s_7_21: u64 = D_read(state, tracer, s_7_20);
        // D s_7_22: cast zx s_7_13 -> bv
        let s_7_22: Bits = Bits::new(s_7_13 as u128, 64u16);
        // D s_7_23: cast zx s_7_21 -> bv
        let s_7_23: Bits = Bits::new(s_7_21 as u128, 64u16);
        // D s_7_24: and s_7_22 s_7_23
        let s_7_24: Bits = ((s_7_22) & (s_7_23));
        // D s_7_25: cast reint s_7_24 -> u64
        let s_7_25: u64 = (s_7_24.value() as u64);
        // D s_7_26: read-var d:i64
        let s_7_26: i64 = fn_state.d;
        // D s_7_27: cast zx s_7_26 -> i
        let s_7_27: i128 = (i128::try_from(s_7_26).unwrap());
        // D s_7_28: read-var r:i64
        let s_7_28: i64 = fn_state.r;
        // D s_7_29: cast zx s_7_28 -> i
        let s_7_29: i128 = (i128::try_from(s_7_28).unwrap());
        // D s_7_30: add s_7_27 s_7_29
        let s_7_30: i128 = (s_7_27 + s_7_29);
        // D s_7_31: cast reint s_7_30 -> i64
        let s_7_31: i64 = (s_7_30 as i64);
        // D s_7_32: cast zx s_7_31 -> i
        let s_7_32: i128 = (i128::try_from(s_7_31).unwrap());
        // D s_7_33: call D_read(s_7_32)
        let s_7_33: u64 = D_read(state, tracer, s_7_32);
        // D s_7_34: read-var m:i64
        let s_7_34: i64 = fn_state.m;
        // D s_7_35: cast zx s_7_34 -> i
        let s_7_35: i128 = (i128::try_from(s_7_34).unwrap());
        // D s_7_36: read-var r:i64
        let s_7_36: i64 = fn_state.r;
        // D s_7_37: cast zx s_7_36 -> i
        let s_7_37: i128 = (i128::try_from(s_7_36).unwrap());
        // D s_7_38: add s_7_35 s_7_37
        let s_7_38: i128 = (s_7_35 + s_7_37);
        // D s_7_39: cast reint s_7_38 -> i64
        let s_7_39: i64 = (s_7_38 as i64);
        // D s_7_40: cast zx s_7_39 -> i
        let s_7_40: i128 = (i128::try_from(s_7_39).unwrap());
        // D s_7_41: call D_read(s_7_40)
        let s_7_41: u64 = D_read(state, tracer, s_7_40);
        // D s_7_42: cast zx s_7_41 -> bv
        let s_7_42: Bits = Bits::new(s_7_41 as u128, 64u16);
        // D s_7_43: not s_7_42
        let s_7_43: Bits = !s_7_42;
        // D s_7_44: cast reint s_7_43 -> u64
        let s_7_44: u64 = (s_7_43.value() as u64);
        // D s_7_45: cast zx s_7_33 -> bv
        let s_7_45: Bits = Bits::new(s_7_33 as u128, 64u16);
        // D s_7_46: cast zx s_7_44 -> bv
        let s_7_46: Bits = Bits::new(s_7_44 as u128, 64u16);
        // D s_7_47: and s_7_45 s_7_46
        let s_7_47: Bits = ((s_7_45) & (s_7_46));
        // D s_7_48: cast reint s_7_47 -> u64
        let s_7_48: u64 = (s_7_47.value() as u64);
        // D s_7_49: cast zx s_7_25 -> bv
        let s_7_49: Bits = Bits::new(s_7_25 as u128, 64u16);
        // D s_7_50: cast zx s_7_48 -> bv
        let s_7_50: Bits = Bits::new(s_7_48 as u128, 64u16);
        // D s_7_51: or s_7_49 s_7_50
        let s_7_51: Bits = ((s_7_49) | (s_7_50));
        // D s_7_52: cast reint s_7_51 -> u64
        let s_7_52: u64 = (s_7_51.value() as u64);
        // D s_7_53: cast zx s_7_5 -> i
        let s_7_53: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_54: call D_set(s_7_53, s_7_52)
        let s_7_54: () = D_set(state, tracer, s_7_53, s_7_52);
        // N s_7_55: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #2u : u32
        let s_8_0: u32 = 2;
        // D s_8_1: read-var operation:u32
        let s_8_1: u32 = fn_state.operation;
        // D s_8_2: cmp-eq s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) == (s_8_1));
        // D s_8_3: not s_8_2
        let s_8_3: bool = !s_8_2;
        // N s_8_4: branch s_8_3 b10 b9
        if s_8_3 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var d:i64
        let s_9_0: i64 = fn_state.d;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: read-var r:i64
        let s_9_2: i64 = fn_state.r;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: add s_9_1 s_9_3
        let s_9_4: i128 = (s_9_1 + s_9_3);
        // D s_9_5: cast reint s_9_4 -> i64
        let s_9_5: i64 = (s_9_4 as i64);
        // D s_9_6: read-var n:i64
        let s_9_6: i64 = fn_state.n;
        // D s_9_7: cast zx s_9_6 -> i
        let s_9_7: i128 = (i128::try_from(s_9_6).unwrap());
        // D s_9_8: read-var r:i64
        let s_9_8: i64 = fn_state.r;
        // D s_9_9: cast zx s_9_8 -> i
        let s_9_9: i128 = (i128::try_from(s_9_8).unwrap());
        // D s_9_10: add s_9_7 s_9_9
        let s_9_10: i128 = (s_9_7 + s_9_9);
        // D s_9_11: cast reint s_9_10 -> i64
        let s_9_11: i64 = (s_9_10 as i64);
        // D s_9_12: cast zx s_9_11 -> i
        let s_9_12: i128 = (i128::try_from(s_9_11).unwrap());
        // D s_9_13: call D_read(s_9_12)
        let s_9_13: u64 = D_read(state, tracer, s_9_12);
        // D s_9_14: read-var d:i64
        let s_9_14: i64 = fn_state.d;
        // D s_9_15: cast zx s_9_14 -> i
        let s_9_15: i128 = (i128::try_from(s_9_14).unwrap());
        // D s_9_16: read-var r:i64
        let s_9_16: i64 = fn_state.r;
        // D s_9_17: cast zx s_9_16 -> i
        let s_9_17: i128 = (i128::try_from(s_9_16).unwrap());
        // D s_9_18: add s_9_15 s_9_17
        let s_9_18: i128 = (s_9_15 + s_9_17);
        // D s_9_19: cast reint s_9_18 -> i64
        let s_9_19: i64 = (s_9_18 as i64);
        // D s_9_20: cast zx s_9_19 -> i
        let s_9_20: i128 = (i128::try_from(s_9_19).unwrap());
        // D s_9_21: call D_read(s_9_20)
        let s_9_21: u64 = D_read(state, tracer, s_9_20);
        // D s_9_22: cast zx s_9_13 -> bv
        let s_9_22: Bits = Bits::new(s_9_13 as u128, 64u16);
        // D s_9_23: cast zx s_9_21 -> bv
        let s_9_23: Bits = Bits::new(s_9_21 as u128, 64u16);
        // D s_9_24: and s_9_22 s_9_23
        let s_9_24: Bits = ((s_9_22) & (s_9_23));
        // D s_9_25: cast reint s_9_24 -> u64
        let s_9_25: u64 = (s_9_24.value() as u64);
        // D s_9_26: read-var m:i64
        let s_9_26: i64 = fn_state.m;
        // D s_9_27: cast zx s_9_26 -> i
        let s_9_27: i128 = (i128::try_from(s_9_26).unwrap());
        // D s_9_28: read-var r:i64
        let s_9_28: i64 = fn_state.r;
        // D s_9_29: cast zx s_9_28 -> i
        let s_9_29: i128 = (i128::try_from(s_9_28).unwrap());
        // D s_9_30: add s_9_27 s_9_29
        let s_9_30: i128 = (s_9_27 + s_9_29);
        // D s_9_31: cast reint s_9_30 -> i64
        let s_9_31: i64 = (s_9_30 as i64);
        // D s_9_32: cast zx s_9_31 -> i
        let s_9_32: i128 = (i128::try_from(s_9_31).unwrap());
        // D s_9_33: call D_read(s_9_32)
        let s_9_33: u64 = D_read(state, tracer, s_9_32);
        // D s_9_34: read-var d:i64
        let s_9_34: i64 = fn_state.d;
        // D s_9_35: cast zx s_9_34 -> i
        let s_9_35: i128 = (i128::try_from(s_9_34).unwrap());
        // D s_9_36: read-var r:i64
        let s_9_36: i64 = fn_state.r;
        // D s_9_37: cast zx s_9_36 -> i
        let s_9_37: i128 = (i128::try_from(s_9_36).unwrap());
        // D s_9_38: add s_9_35 s_9_37
        let s_9_38: i128 = (s_9_35 + s_9_37);
        // D s_9_39: cast reint s_9_38 -> i64
        let s_9_39: i64 = (s_9_38 as i64);
        // D s_9_40: cast zx s_9_39 -> i
        let s_9_40: i128 = (i128::try_from(s_9_39).unwrap());
        // D s_9_41: call D_read(s_9_40)
        let s_9_41: u64 = D_read(state, tracer, s_9_40);
        // D s_9_42: cast zx s_9_41 -> bv
        let s_9_42: Bits = Bits::new(s_9_41 as u128, 64u16);
        // D s_9_43: not s_9_42
        let s_9_43: Bits = !s_9_42;
        // D s_9_44: cast reint s_9_43 -> u64
        let s_9_44: u64 = (s_9_43.value() as u64);
        // D s_9_45: cast zx s_9_33 -> bv
        let s_9_45: Bits = Bits::new(s_9_33 as u128, 64u16);
        // D s_9_46: cast zx s_9_44 -> bv
        let s_9_46: Bits = Bits::new(s_9_44 as u128, 64u16);
        // D s_9_47: and s_9_45 s_9_46
        let s_9_47: Bits = ((s_9_45) & (s_9_46));
        // D s_9_48: cast reint s_9_47 -> u64
        let s_9_48: u64 = (s_9_47.value() as u64);
        // D s_9_49: cast zx s_9_25 -> bv
        let s_9_49: Bits = Bits::new(s_9_25 as u128, 64u16);
        // D s_9_50: cast zx s_9_48 -> bv
        let s_9_50: Bits = Bits::new(s_9_48 as u128, 64u16);
        // D s_9_51: or s_9_49 s_9_50
        let s_9_51: Bits = ((s_9_49) | (s_9_50));
        // D s_9_52: cast reint s_9_51 -> u64
        let s_9_52: u64 = (s_9_51.value() as u64);
        // D s_9_53: cast zx s_9_5 -> i
        let s_9_53: i128 = (i128::try_from(s_9_5).unwrap());
        // D s_9_54: call D_set(s_9_53, s_9_52)
        let s_9_54: () = D_set(state, tracer, s_9_53, s_9_52);
        // N s_9_55: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: return
        return;
    }
}
