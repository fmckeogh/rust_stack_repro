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
use ALUExceptionReturn::*;
use R_read::*;
use R_set::*;
use ALUWritePC::*;
use AddWithCarry::*;
use common::*;
pub fn execute_aarch32_instrs_SUB_SP_i_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    imm32: u32,
    setflags: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_886870: ProductTyped54bc449dd09e5bd,
        result: u32,
        nzcv: u8,
        d: i64,
        imm32: u32,
        setflags: bool,
    }
    let fn_state = FunctionState {
        d,
        imm32,
        setflags,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #13s : i
        let s_0_0: i128 = 13;
        // S s_0_1: call R_read(s_0_0)
        let s_0_1: u32 = R_read(state, tracer, s_0_0);
        // D s_0_2: read-var imm32:u32
        let s_0_2: u32 = fn_state.imm32;
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_4: not s_0_3
        let s_0_4: Bits = !s_0_3;
        // D s_0_5: cast reint s_0_4 -> u32
        let s_0_5: u32 = (s_0_4.value() as u32);
        // S s_0_6: cast zx s_0_1 -> bv
        let s_0_6: Bits = Bits::new(s_0_1 as u128, 32u16);
        // D s_0_7: cast zx s_0_5 -> bv
        let s_0_7: Bits = Bits::new(s_0_5 as u128, 32u16);
        // C s_0_8: const #1u : u8
        let s_0_8: bool = true;
        // D s_0_9: call AddWithCarry(s_0_6, s_0_7, s_0_8)
        let s_0_9: ProductTyped54bc449dd09e5bd = AddWithCarry(
            state,
            tracer,
            s_0_6,
            s_0_7,
            s_0_8,
        );
        // D s_0_10: write-var gs#886870 <= s_0_9
        fn_state.gs_886870 = s_0_9;
        // D s_0_11: read-var gs#886870.0:struct
        let s_0_11: Bits = fn_state.gs_886870._0;
        // D s_0_12: cast reint s_0_11 -> u32
        let s_0_12: u32 = (s_0_11.value() as u32);
        // D s_0_13: read-var gs#886870.1:struct
        let s_0_13: u8 = fn_state.gs_886870._1;
        // D s_0_14: write-var result <= s_0_12
        fn_state.result = s_0_12;
        // D s_0_15: write-var nzcv <= s_0_13
        fn_state.nzcv = s_0_13;
        // C s_0_16: const #15s : i
        let s_0_16: i128 = 15;
        // D s_0_17: read-var d:i64
        let s_0_17: i64 = fn_state.d;
        // D s_0_18: cast zx s_0_17 -> i
        let s_0_18: i128 = (i128::try_from(s_0_17).unwrap());
        // D s_0_19: cmp-eq s_0_18 s_0_16
        let s_0_19: bool = ((s_0_18) == (s_0_16));
        // N s_0_20: branch s_0_19 b4 b1
        if s_0_19 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var d:i64
        let s_1_0: i64 = fn_state.d;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: read-var result:u32
        let s_1_2: u32 = fn_state.result;
        // D s_1_3: call R_set(s_1_1, s_1_2)
        let s_1_3: () = R_set(state, tracer, s_1_1, s_1_2);
        // D s_1_4: read-var setflags:u8
        let s_1_4: bool = fn_state.setflags;
        // N s_1_5: branch s_1_4 b3 b2
        if s_1_4 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #3s : i
        let s_3_0: i128 = 3;
        // D s_3_1: read-var nzcv:u8
        let s_3_1: u8 = fn_state.nzcv;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 4u16);
        // C s_3_3: const #1s : i64
        let s_3_3: i64 = 1;
        // C s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // C s_3_5: const #0s : i
        let s_3_5: i128 = 0;
        // C s_3_6: add s_3_5 s_3_4
        let s_3_6: i128 = (s_3_5 + s_3_4);
        // D s_3_7: bit-extract s_3_2 s_3_0 s_3_6
        let s_3_7: Bits = (Bits::new(
            ((s_3_2) >> (s_3_0)).value(),
            u16::try_from(s_3_6).unwrap(),
        ));
        // D s_3_8: cast reint s_3_7 -> u8
        let s_3_8: bool = ((s_3_7.value()) != 0);
        // C s_3_9: const #16984u : u32
        let s_3_9: u32 = 16984;
        // N s_3_10: write-reg s_3_9 <= s_3_8
        let s_3_10: () = {
            state.write_register::<bool>(s_3_9 as isize, s_3_8);
            tracer.write_register(s_3_9 as isize, s_3_8);
        };
        // C s_3_11: const #2s : i
        let s_3_11: i128 = 2;
        // D s_3_12: read-var nzcv:u8
        let s_3_12: u8 = fn_state.nzcv;
        // D s_3_13: cast zx s_3_12 -> bv
        let s_3_13: Bits = Bits::new(s_3_12 as u128, 4u16);
        // C s_3_14: const #1s : i64
        let s_3_14: i64 = 1;
        // C s_3_15: cast zx s_3_14 -> i
        let s_3_15: i128 = (i128::try_from(s_3_14).unwrap());
        // C s_3_16: const #0s : i
        let s_3_16: i128 = 0;
        // C s_3_17: add s_3_16 s_3_15
        let s_3_17: i128 = (s_3_16 + s_3_15);
        // D s_3_18: bit-extract s_3_13 s_3_11 s_3_17
        let s_3_18: Bits = (Bits::new(
            ((s_3_13) >> (s_3_11)).value(),
            u16::try_from(s_3_17).unwrap(),
        ));
        // D s_3_19: cast reint s_3_18 -> u8
        let s_3_19: bool = ((s_3_18.value()) != 0);
        // C s_3_20: const #16997u : u32
        let s_3_20: u32 = 16997;
        // N s_3_21: write-reg s_3_20 <= s_3_19
        let s_3_21: () = {
            state.write_register::<bool>(s_3_20 as isize, s_3_19);
            tracer.write_register(s_3_20 as isize, s_3_19);
        };
        // C s_3_22: const #1s : i
        let s_3_22: i128 = 1;
        // D s_3_23: read-var nzcv:u8
        let s_3_23: u8 = fn_state.nzcv;
        // D s_3_24: cast zx s_3_23 -> bv
        let s_3_24: Bits = Bits::new(s_3_23 as u128, 4u16);
        // C s_3_25: const #1s : i64
        let s_3_25: i64 = 1;
        // C s_3_26: cast zx s_3_25 -> i
        let s_3_26: i128 = (i128::try_from(s_3_25).unwrap());
        // C s_3_27: const #0s : i
        let s_3_27: i128 = 0;
        // C s_3_28: add s_3_27 s_3_26
        let s_3_28: i128 = (s_3_27 + s_3_26);
        // D s_3_29: bit-extract s_3_24 s_3_22 s_3_28
        let s_3_29: Bits = (Bits::new(
            ((s_3_24) >> (s_3_22)).value(),
            u16::try_from(s_3_28).unwrap(),
        ));
        // D s_3_30: cast reint s_3_29 -> u8
        let s_3_30: bool = ((s_3_29.value()) != 0);
        // C s_3_31: const #16971u : u32
        let s_3_31: u32 = 16971;
        // N s_3_32: write-reg s_3_31 <= s_3_30
        let s_3_32: () = {
            state.write_register::<bool>(s_3_31 as isize, s_3_30);
            tracer.write_register(s_3_31 as isize, s_3_30);
        };
        // C s_3_33: const #0s : i
        let s_3_33: i128 = 0;
        // D s_3_34: read-var nzcv:u8
        let s_3_34: u8 = fn_state.nzcv;
        // D s_3_35: cast zx s_3_34 -> bv
        let s_3_35: Bits = Bits::new(s_3_34 as u128, 4u16);
        // C s_3_36: const #1s : i64
        let s_3_36: i64 = 1;
        // C s_3_37: cast zx s_3_36 -> i
        let s_3_37: i128 = (i128::try_from(s_3_36).unwrap());
        // C s_3_38: const #0s : i
        let s_3_38: i128 = 0;
        // C s_3_39: add s_3_38 s_3_37
        let s_3_39: i128 = (s_3_38 + s_3_37);
        // D s_3_40: bit-extract s_3_35 s_3_33 s_3_39
        let s_3_40: Bits = (Bits::new(
            ((s_3_35) >> (s_3_33)).value(),
            u16::try_from(s_3_39).unwrap(),
        ));
        // D s_3_41: cast reint s_3_40 -> u8
        let s_3_41: bool = ((s_3_40.value()) != 0);
        // C s_3_42: const #16996u : u32
        let s_3_42: u32 = 16996;
        // N s_3_43: write-reg s_3_42 <= s_3_41
        let s_3_43: () = {
            state.write_register::<bool>(s_3_42 as isize, s_3_41);
            tracer.write_register(s_3_42 as isize, s_3_41);
        };
        // N s_3_44: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var setflags:u8
        let s_4_0: bool = fn_state.setflags;
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
        // D s_5_0: read-var result:u32
        let s_5_0: u32 = fn_state.result;
        // D s_5_1: call ALUWritePC(s_5_0)
        let s_5_1: () = ALUWritePC(state, tracer, s_5_0);
        // N s_5_2: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var result:u32
        let s_6_0: u32 = fn_state.result;
        // D s_6_1: call ALUExceptionReturn(s_6_0)
        let s_6_1: () = ALUExceptionReturn(state, tracer, s_6_0);
        // N s_6_2: return
        return;
    }
}
