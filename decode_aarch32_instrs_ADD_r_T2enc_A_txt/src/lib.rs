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
use LastInITBlock::*;
use ConditionPassed::*;
use execute_aarch32_instrs_ADD_r_Op_A_txt::*;
use InITBlock::*;
use common::*;
pub fn decode_aarch32_instrs_ADD_r_T2enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    DN: bool,
    Rm: u8,
    Rdn: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_295544: bool,
        m: i64,
        gs_295542: bool,
        shift_nshadow_7113: i128,
        n: i64,
        gs_295528: bool,
        d: i64,
        shift_t: u32,
        gs_295545: bool,
        DN: bool,
        Rm: u8,
        Rdn: u8,
    }
    let fn_state = FunctionState {
        DN,
        Rm,
        Rdn,
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
        // D s_2_0: read-var DN:u8
        let s_2_0: bool = fn_state.DN;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // D s_2_2: read-var Rdn:u8
        let s_2_2: u8 = fn_state.Rdn;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 3u16);
        // D s_2_4: cast reint s_2_1 -> u128
        let s_2_4: u128 = (s_2_1.value() as u128);
        // D s_2_5: size-of s_2_1
        let s_2_5: u16 = s_2_1.length();
        // D s_2_6: cast reint s_2_3 -> u128
        let s_2_6: u128 = (s_2_3.value() as u128);
        // D s_2_7: size-of s_2_3
        let s_2_7: u16 = s_2_3.length();
        // D s_2_8: lsl s_2_4 s_2_7
        let s_2_8: u128 = s_2_4 << s_2_7;
        // D s_2_9: or s_2_8 s_2_6
        let s_2_9: u128 = ((s_2_8) | (s_2_6));
        // D s_2_10: add s_2_5 s_2_7
        let s_2_10: u16 = (s_2_5 + s_2_7);
        // D s_2_11: create-bits s_2_9 s_2_10
        let s_2_11: Bits = Bits::new(s_2_9, s_2_10);
        // D s_2_12: cast reint s_2_11 -> u8
        let s_2_12: u8 = (s_2_11.value() as u8);
        // D s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 4u16);
        // C s_2_14: const #13u : u8
        let s_2_14: u8 = 13;
        // C s_2_15: cast zx s_2_14 -> bv
        let s_2_15: Bits = Bits::new(s_2_14 as u128, 4u16);
        // D s_2_16: cmp-eq s_2_13 s_2_15
        let s_2_16: bool = ((s_2_13) == (s_2_15));
        // N s_2_17: branch s_2_16 b20 b3
        if s_2_16 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var Rm:u8
        let s_3_0: u8 = fn_state.Rm;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 4u16);
        // C s_3_2: const #13u : u8
        let s_3_2: u8 = 13;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 4u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // D s_3_5: write-var gs#295528 <= s_3_4
        fn_state.gs_295528 = s_3_4;
        // N s_3_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#295528:u8
        let s_4_0: bool = fn_state.gs_295528;
        // N s_4_1: branch s_4_0 b19 b5
        if s_4_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var DN:u8
        let s_5_0: bool = fn_state.DN;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // D s_5_2: read-var Rdn:u8
        let s_5_2: u8 = fn_state.Rdn;
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 3u16);
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
        let s_5_13: Bits = Bits::new(s_5_12 as u128, 4u16);
        // D s_5_14: cast zx s_5_13 -> i
        let s_5_14: i128 = (s_5_13.value() as i128);
        // D s_5_15: cast reint s_5_14 -> i64
        let s_5_15: i64 = (s_5_14 as i64);
        // D s_5_16: write-var d <= s_5_15
        fn_state.d = s_5_15;
        // D s_5_17: read-var d:i64
        let s_5_17: i64 = fn_state.d;
        // D s_5_18: write-var n <= s_5_17
        fn_state.n = s_5_17;
        // D s_5_19: read-var Rm:u8
        let s_5_19: u8 = fn_state.Rm;
        // D s_5_20: cast zx s_5_19 -> bv
        let s_5_20: Bits = Bits::new(s_5_19 as u128, 4u16);
        // D s_5_21: cast zx s_5_20 -> i
        let s_5_21: i128 = (s_5_20.value() as i128);
        // D s_5_22: cast reint s_5_21 -> i64
        let s_5_22: i64 = (s_5_21 as i64);
        // D s_5_23: write-var m <= s_5_22
        fn_state.m = s_5_22;
        // C s_5_24: const #0s : i
        let s_5_24: i128 = 0;
        // C s_5_25: const #0u : u32
        let s_5_25: u32 = 0;
        // D s_5_26: create-product struct = ["s_5_25", "s_5_24"]
        let s_5_26: ProductType396b95aa74979079 = ProductType396b95aa74979079 {
            _0: s_5_25,
            _1: s_5_24,
        };
        // D s_5_27: extract-field s_5_26.0
        let s_5_27: u32 = s_5_26._0;
        // C s_5_28: const #0u : u32
        let s_5_28: u32 = 0;
        // D s_5_29: create-product struct = ["s_5_28", "s_5_24"]
        let s_5_29: ProductType396b95aa74979079 = ProductType396b95aa74979079 {
            _0: s_5_28,
            _1: s_5_24,
        };
        // D s_5_30: extract-field s_5_29.1
        let s_5_30: i128 = s_5_29._1;
        // D s_5_31: cast reint s_5_30 -> i64
        let s_5_31: i64 = (s_5_30 as i64);
        // D s_5_32: write-var shift_t <= s_5_27
        fn_state.shift_t = s_5_27;
        // D s_5_33: cast zx s_5_31 -> i
        let s_5_33: i128 = (i128::try_from(s_5_31).unwrap());
        // D s_5_34: write-var shift_nshadow#7113 <= s_5_33
        fn_state.shift_nshadow_7113 = s_5_33;
        // C s_5_35: const #15s : i
        let s_5_35: i128 = 15;
        // D s_5_36: read-var n:i64
        let s_5_36: i64 = fn_state.n;
        // D s_5_37: cast zx s_5_36 -> i
        let s_5_37: i128 = (i128::try_from(s_5_36).unwrap());
        // D s_5_38: cmp-eq s_5_37 s_5_35
        let s_5_38: bool = ((s_5_37) == (s_5_35));
        // N s_5_39: branch s_5_38 b18 b6
        if s_5_38 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#295542 <= s_6_0
        fn_state.gs_295542 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#295542:u8
        let s_7_0: bool = fn_state.gs_295542;
        // N s_7_1: branch s_7_0 b17 b8
        if s_7_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #15s : i
        let s_8_0: i128 = 15;
        // D s_8_1: read-var d:i64
        let s_8_1: i64 = fn_state.d;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: cmp-eq s_8_2 s_8_0
        let s_8_3: bool = ((s_8_2) == (s_8_0));
        // N s_8_4: branch s_8_3 b16 b9
        if s_8_3 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#295544 <= s_9_0
        fn_state.gs_295544 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#295544:u8
        let s_10_0: bool = fn_state.gs_295544;
        // N s_10_1: branch s_10_0 b15 b11
        if s_10_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#295545 <= s_11_0
        fn_state.gs_295545 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#295545:u8
        let s_12_0: bool = fn_state.gs_295545;
        // N s_12_1: branch s_12_0 b14 b13
        if s_12_0 {
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
        // D s_13_0: read-var d:i64
        let s_13_0: i64 = fn_state.d;
        // D s_13_1: read-var m:i64
        let s_13_1: i64 = fn_state.m;
        // D s_13_2: read-var n:i64
        let s_13_2: i64 = fn_state.n;
        // C s_13_3: const #0u : u8
        let s_13_3: bool = false;
        // D s_13_4: read-var shift_nshadow#7113:i
        let s_13_4: i128 = fn_state.shift_nshadow_7113;
        // D s_13_5: read-var shift_t:u32
        let s_13_5: u32 = fn_state.shift_t;
        // D s_13_6: call execute_aarch32_instrs_ADD_r_Op_A_txt(s_13_0, s_13_1, s_13_2, s_13_3, s_13_4, s_13_5)
        let s_13_6: () = execute_aarch32_instrs_ADD_r_Op_A_txt(
            state,
            tracer,
            s_13_0,
            s_13_1,
            s_13_2,
            s_13_3,
            s_13_4,
            s_13_5,
        );
        // N s_13_7: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: panic
        panic!("{:?}", ());
        // N s_14_1: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call LastInITBlock(s_15_0)
        let s_15_1: bool = LastInITBlock(state, tracer, s_15_0);
        // S s_15_2: not s_15_1
        let s_15_2: bool = !s_15_1;
        // D s_15_3: write-var gs#295545 <= s_15_2
        fn_state.gs_295545 = s_15_2;
        // N s_15_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call InITBlock(s_16_0)
        let s_16_1: bool = InITBlock(state, tracer, s_16_0);
        // D s_16_2: write-var gs#295544 <= s_16_1
        fn_state.gs_295544 = s_16_1;
        // N s_16_3: jump b10
        return block_10(state, tracer, fn_state);
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
        // C s_18_0: const #15s : i
        let s_18_0: i128 = 15;
        // D s_18_1: read-var m:i64
        let s_18_1: i64 = fn_state.m;
        // D s_18_2: cast zx s_18_1 -> i
        let s_18_2: i128 = (i128::try_from(s_18_1).unwrap());
        // D s_18_3: cmp-eq s_18_2 s_18_0
        let s_18_3: bool = ((s_18_2) == (s_18_0));
        // D s_18_4: write-var gs#295542 <= s_18_3
        fn_state.gs_295542 = s_18_3;
        // N s_18_5: jump b7
        return block_7(state, tracer, fn_state);
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
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#295528 <= s_20_0
        fn_state.gs_295528 = s_20_0;
        // N s_20_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
