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
use DecodeImmShift::*;
use execute_aarch32_instrs_MVN_r_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_MVN_r_T2enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    S: bool,
    imm3: u8,
    Rd: u8,
    imm2: u8,
    stype: u8,
    Rm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        shift_nshadow_7231: i128,
        setflags: bool,
        gs_298899: bool,
        ga_345269: ProductType396b95aa74979079,
        shift_t: u32,
        d: i64,
        S: bool,
        imm3: u8,
        Rd: u8,
        imm2: u8,
        stype: u8,
        Rm: u8,
    }
    let fn_state = FunctionState {
        S,
        imm3,
        Rd,
        imm2,
        stype,
        Rm,
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
        // D s_2_0: read-var Rd:u8
        let s_2_0: u8 = fn_state.Rd;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // D s_2_4: write-var d <= s_2_3
        fn_state.d = s_2_3;
        // D s_2_5: read-var Rm:u8
        let s_2_5: u8 = fn_state.Rm;
        // D s_2_6: cast zx s_2_5 -> bv
        let s_2_6: Bits = Bits::new(s_2_5 as u128, 4u16);
        // D s_2_7: cast zx s_2_6 -> i
        let s_2_7: i128 = (s_2_6.value() as i128);
        // D s_2_8: cast reint s_2_7 -> i64
        let s_2_8: i64 = (s_2_7 as i64);
        // D s_2_9: write-var m <= s_2_8
        fn_state.m = s_2_8;
        // D s_2_10: read-var S:u8
        let s_2_10: bool = fn_state.S;
        // D s_2_11: cast zx s_2_10 -> bv
        let s_2_11: Bits = Bits::new(s_2_10 as u128, 1u16);
        // C s_2_12: const #1u : u8
        let s_2_12: bool = true;
        // C s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 1u16);
        // D s_2_14: cmp-eq s_2_11 s_2_13
        let s_2_14: bool = ((s_2_11) == (s_2_13));
        // D s_2_15: write-var setflags <= s_2_14
        fn_state.setflags = s_2_14;
        // D s_2_16: read-var imm3:u8
        let s_2_16: u8 = fn_state.imm3;
        // D s_2_17: cast zx s_2_16 -> bv
        let s_2_17: Bits = Bits::new(s_2_16 as u128, 3u16);
        // D s_2_18: read-var imm2:u8
        let s_2_18: u8 = fn_state.imm2;
        // D s_2_19: cast zx s_2_18 -> bv
        let s_2_19: Bits = Bits::new(s_2_18 as u128, 2u16);
        // D s_2_20: cast reint s_2_17 -> u128
        let s_2_20: u128 = (s_2_17.value() as u128);
        // D s_2_21: size-of s_2_17
        let s_2_21: u16 = s_2_17.length();
        // D s_2_22: cast reint s_2_19 -> u128
        let s_2_22: u128 = (s_2_19.value() as u128);
        // D s_2_23: size-of s_2_19
        let s_2_23: u16 = s_2_19.length();
        // D s_2_24: lsl s_2_20 s_2_23
        let s_2_24: u128 = s_2_20 << s_2_23;
        // D s_2_25: or s_2_24 s_2_22
        let s_2_25: u128 = ((s_2_24) | (s_2_22));
        // D s_2_26: add s_2_21 s_2_23
        let s_2_26: u16 = (s_2_21 + s_2_23);
        // D s_2_27: create-bits s_2_25 s_2_26
        let s_2_27: Bits = Bits::new(s_2_25, s_2_26);
        // D s_2_28: cast reint s_2_27 -> u8
        let s_2_28: u8 = (s_2_27.value() as u8);
        // D s_2_29: read-var stype:u8
        let s_2_29: u8 = fn_state.stype;
        // D s_2_30: call DecodeImmShift(s_2_29, s_2_28)
        let s_2_30: ProductType396b95aa74979079 = DecodeImmShift(
            state,
            tracer,
            s_2_29,
            s_2_28,
        );
        // D s_2_31: write-var ga#345269 <= s_2_30
        fn_state.ga_345269 = s_2_30;
        // D s_2_32: read-var ga#345269.0:struct
        let s_2_32: u32 = fn_state.ga_345269._0;
        // D s_2_33: read-var ga#345269.1:struct
        let s_2_33: i128 = fn_state.ga_345269._1;
        // D s_2_34: write-var shift_t <= s_2_32
        fn_state.shift_t = s_2_32;
        // D s_2_35: write-var shift_nshadow#7231 <= s_2_33
        fn_state.shift_nshadow_7231 = s_2_33;
        // C s_2_36: const #15s : i
        let s_2_36: i128 = 15;
        // D s_2_37: read-var d:i64
        let s_2_37: i64 = fn_state.d;
        // D s_2_38: cast zx s_2_37 -> i
        let s_2_38: i128 = (i128::try_from(s_2_37).unwrap());
        // D s_2_39: cmp-eq s_2_38 s_2_36
        let s_2_39: bool = ((s_2_38) == (s_2_36));
        // N s_2_40: branch s_2_39 b7 b3
        if s_2_39 {
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
        // C s_3_0: const #15s : i
        let s_3_0: i128 = 15;
        // D s_3_1: read-var m:i64
        let s_3_1: i64 = fn_state.m;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: cmp-eq s_3_2 s_3_0
        let s_3_3: bool = ((s_3_2) == (s_3_0));
        // D s_3_4: write-var gs#298899 <= s_3_3
        fn_state.gs_298899 = s_3_3;
        // N s_3_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#298899:u8
        let s_4_0: bool = fn_state.gs_298899;
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
        // D s_5_0: read-var shift_t:u32
        let s_5_0: u32 = fn_state.shift_t;
        // D s_5_1: read-var shift_nshadow#7231:i
        let s_5_1: i128 = fn_state.shift_nshadow_7231;
        // D s_5_2: read-var d:i64
        let s_5_2: i64 = fn_state.d;
        // D s_5_3: read-var m:i64
        let s_5_3: i64 = fn_state.m;
        // D s_5_4: read-var setflags:u8
        let s_5_4: bool = fn_state.setflags;
        // D s_5_5: call execute_aarch32_instrs_MVN_r_Op_A_txt(s_5_2, s_5_3, s_5_4, s_5_1, s_5_0)
        let s_5_5: () = execute_aarch32_instrs_MVN_r_Op_A_txt(
            state,
            tracer,
            s_5_2,
            s_5_3,
            s_5_4,
            s_5_1,
            s_5_0,
        );
        // N s_5_6: return
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
        // D s_7_1: write-var gs#298899 <= s_7_0
        fn_state.gs_298899 = s_7_0;
        // N s_7_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
