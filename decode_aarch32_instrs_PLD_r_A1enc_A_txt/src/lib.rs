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
use execute_aarch32_instrs_PLD_r_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_PLD_r_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    U: bool,
    R: bool,
    Rn: u8,
    imm5: u8,
    stype: u8,
    Rm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_299195: bool,
        m: i64,
        is_pldw: bool,
        ga_345457: ProductType396b95aa74979079,
        gs_299196: bool,
        shift_t: u32,
        shift_nshadow_7242: i128,
        n: i64,
        add: bool,
        U: bool,
        R: bool,
        Rn: u8,
        imm5: u8,
        stype: u8,
        Rm: u8,
    }
    let fn_state = FunctionState {
        U,
        R,
        Rn,
        imm5,
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
        // D s_2_0: read-var Rn:u8
        let s_2_0: u8 = fn_state.Rn;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // D s_2_4: write-var n <= s_2_3
        fn_state.n = s_2_3;
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
        // D s_2_10: read-var U:u8
        let s_2_10: bool = fn_state.U;
        // D s_2_11: cast zx s_2_10 -> bv
        let s_2_11: Bits = Bits::new(s_2_10 as u128, 1u16);
        // C s_2_12: const #1u : u8
        let s_2_12: bool = true;
        // C s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 1u16);
        // D s_2_14: cmp-eq s_2_11 s_2_13
        let s_2_14: bool = ((s_2_11) == (s_2_13));
        // D s_2_15: write-var add <= s_2_14
        fn_state.add = s_2_14;
        // D s_2_16: read-var R:u8
        let s_2_16: bool = fn_state.R;
        // D s_2_17: cast zx s_2_16 -> bv
        let s_2_17: Bits = Bits::new(s_2_16 as u128, 1u16);
        // C s_2_18: const #0u : u8
        let s_2_18: bool = false;
        // C s_2_19: cast zx s_2_18 -> bv
        let s_2_19: Bits = Bits::new(s_2_18 as u128, 1u16);
        // D s_2_20: cmp-eq s_2_17 s_2_19
        let s_2_20: bool = ((s_2_17) == (s_2_19));
        // D s_2_21: write-var is_pldw <= s_2_20
        fn_state.is_pldw = s_2_20;
        // D s_2_22: read-var stype:u8
        let s_2_22: u8 = fn_state.stype;
        // D s_2_23: read-var imm5:u8
        let s_2_23: u8 = fn_state.imm5;
        // D s_2_24: call DecodeImmShift(s_2_22, s_2_23)
        let s_2_24: ProductType396b95aa74979079 = DecodeImmShift(
            state,
            tracer,
            s_2_22,
            s_2_23,
        );
        // D s_2_25: write-var ga#345457 <= s_2_24
        fn_state.ga_345457 = s_2_24;
        // D s_2_26: read-var ga#345457.0:struct
        let s_2_26: u32 = fn_state.ga_345457._0;
        // D s_2_27: read-var ga#345457.1:struct
        let s_2_27: i128 = fn_state.ga_345457._1;
        // D s_2_28: write-var shift_t <= s_2_26
        fn_state.shift_t = s_2_26;
        // D s_2_29: write-var shift_nshadow#7242 <= s_2_27
        fn_state.shift_nshadow_7242 = s_2_27;
        // C s_2_30: const #15s : i
        let s_2_30: i128 = 15;
        // D s_2_31: read-var m:i64
        let s_2_31: i64 = fn_state.m;
        // D s_2_32: cast zx s_2_31 -> i
        let s_2_32: i128 = (i128::try_from(s_2_31).unwrap());
        // D s_2_33: cmp-eq s_2_32 s_2_30
        let s_2_33: bool = ((s_2_32) == (s_2_30));
        // N s_2_34: branch s_2_33 b10 b3
        if s_2_33 {
            return block_10(state, tracer, fn_state);
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
        // D s_3_1: read-var n:i64
        let s_3_1: i64 = fn_state.n;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: cmp-eq s_3_2 s_3_0
        let s_3_3: bool = ((s_3_2) == (s_3_0));
        // N s_3_4: branch s_3_3 b9 b4
        if s_3_3 {
            return block_9(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#299195 <= s_4_0
        fn_state.gs_299195 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#299195:u8
        let s_5_0: bool = fn_state.gs_299195;
        // D s_5_1: write-var gs#299196 <= s_5_0
        fn_state.gs_299196 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#299196:u8
        let s_6_0: bool = fn_state.gs_299196;
        // N s_6_1: branch s_6_0 b8 b7
        if s_6_0 {
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
        // D s_7_0: read-var shift_t:u32
        let s_7_0: u32 = fn_state.shift_t;
        // D s_7_1: read-var shift_nshadow#7242:i
        let s_7_1: i128 = fn_state.shift_nshadow_7242;
        // D s_7_2: read-var add:u8
        let s_7_2: bool = fn_state.add;
        // D s_7_3: read-var is_pldw:u8
        let s_7_3: bool = fn_state.is_pldw;
        // D s_7_4: read-var m:i64
        let s_7_4: i64 = fn_state.m;
        // D s_7_5: read-var n:i64
        let s_7_5: i64 = fn_state.n;
        // D s_7_6: call execute_aarch32_instrs_PLD_r_Op_A_txt(s_7_2, s_7_3, s_7_4, s_7_5, s_7_1, s_7_0)
        let s_7_6: () = execute_aarch32_instrs_PLD_r_Op_A_txt(
            state,
            tracer,
            s_7_2,
            s_7_3,
            s_7_4,
            s_7_5,
            s_7_1,
            s_7_0,
        );
        // N s_7_7: return
        return;
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
        // D s_9_0: read-var is_pldw:u8
        let s_9_0: bool = fn_state.is_pldw;
        // D s_9_1: write-var gs#299195 <= s_9_0
        fn_state.gs_299195 = s_9_0;
        // N s_9_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // D s_10_1: write-var gs#299196 <= s_10_0
        fn_state.gs_299196 = s_10_0;
        // N s_10_2: jump b6
        return block_6(state, tracer, fn_state);
    }
}
