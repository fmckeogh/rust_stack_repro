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
use neq_int::*;
use AArch32_SetLSInstructionSyndrome::*;
use R_read::*;
use R_set::*;
use MemU_unpriv_set::*;
use common::*;
pub fn execute_aarch32_instrs_STRHT_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    add: bool,
    imm32: u32,
    m: i128,
    n: i64,
    postindex: bool,
    register_form: bool,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_302895: bool,
        offset: u32,
        offset_addr: u32,
        address: u32,
        add: bool,
        imm32: u32,
        m: i128,
        n: i64,
        postindex: bool,
        register_form: bool,
        t: i64,
    }
    let fn_state = FunctionState {
        add,
        imm32,
        m,
        n,
        postindex,
        register_form,
        t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 2u16);
        // C s_0_3: const #432u : u32
        let s_0_3: u32 = 432;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // D s_0_6: cmp-eq s_0_2 s_0_5
        let s_0_6: bool = ((s_0_2) == (s_0_5));
        // N s_0_7: branch s_0_6 b20 b1
        if s_0_6 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var register_form:u8
        let s_1_0: bool = fn_state.register_form;
        // N s_1_1: branch s_1_0 b19 b2
        if s_1_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var imm32:u32
        let s_2_0: u32 = fn_state.imm32;
        // D s_2_1: write-var offset <= s_2_0
        fn_state.offset = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var add:u8
        let s_3_0: bool = fn_state.add;
        // N s_3_1: branch s_3_0 b18 b4
        if s_3_0 {
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
        // D s_4_0: read-var n:i64
        let s_4_0: i64 = fn_state.n;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: call R_read(s_4_1)
        let s_4_2: u32 = R_read(state, tracer, s_4_1);
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 32u16);
        // D s_4_4: read-var offset:u32
        let s_4_4: u32 = fn_state.offset;
        // D s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 32u16);
        // D s_4_6: sub s_4_3 s_4_5
        let s_4_6: Bits = ((s_4_3) - (s_4_5));
        // D s_4_7: cast reint s_4_6 -> u32
        let s_4_7: u32 = (s_4_6.value() as u32);
        // D s_4_8: write-var offset_addr <= s_4_7
        fn_state.offset_addr = s_4_7;
        // N s_4_9: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var postindex:u8
        let s_5_0: bool = fn_state.postindex;
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
        // D s_6_0: read-var offset_addr:u32
        let s_6_0: u32 = fn_state.offset_addr;
        // D s_6_1: write-var address <= s_6_0
        fn_state.address = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #15s : i
        let s_7_0: i128 = 15;
        // D s_7_1: read-var t:i64
        let s_7_1: i64 = fn_state.t;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: call neq_int(s_7_2, s_7_0)
        let s_7_3: bool = neq_int(state, tracer, s_7_2, s_7_0);
        // N s_7_4: branch s_7_3 b16 b8
        if s_7_3 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#302895 <= s_8_0
        fn_state.gs_302895 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#302895:u8
        let s_9_0: bool = fn_state.gs_302895;
        // N s_9_1: branch s_9_0 b15 b10
        if s_9_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var t:i64
        let s_11_0: i64 = fn_state.t;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: call R_read(s_11_1)
        let s_11_2: u32 = R_read(state, tracer, s_11_1);
        // C s_11_3: const #0s : i
        let s_11_3: i128 = 0;
        // D s_11_4: cast zx s_11_2 -> bv
        let s_11_4: Bits = Bits::new(s_11_2 as u128, 32u16);
        // C s_11_5: const #1s : i64
        let s_11_5: i64 = 1;
        // C s_11_6: cast zx s_11_5 -> i
        let s_11_6: i128 = (i128::try_from(s_11_5).unwrap());
        // C s_11_7: const #15s : i
        let s_11_7: i128 = 15;
        // C s_11_8: add s_11_7 s_11_6
        let s_11_8: i128 = (s_11_7 + s_11_6);
        // D s_11_9: bit-extract s_11_4 s_11_3 s_11_8
        let s_11_9: Bits = (Bits::new(
            ((s_11_4) >> (s_11_3)).value(),
            u16::try_from(s_11_8).unwrap(),
        ));
        // D s_11_10: cast reint s_11_9 -> u16
        let s_11_10: u16 = (s_11_9.value() as u16);
        // C s_11_11: const #2s : i
        let s_11_11: i128 = 2;
        // D s_11_12: cast zx s_11_10 -> bv
        let s_11_12: Bits = Bits::new(s_11_10 as u128, 16u16);
        // D s_11_13: read-var address:u32
        let s_11_13: u32 = fn_state.address;
        // D s_11_14: call MemU_unpriv_set(s_11_13, s_11_11, s_11_12)
        let s_11_14: () = MemU_unpriv_set(state, tracer, s_11_13, s_11_11, s_11_12);
        // N s_11_15: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var postindex:u8
        let s_12_0: bool = fn_state.postindex;
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
        // N s_13_0: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var n:i64
        let s_14_0: i64 = fn_state.n;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: read-var offset_addr:u32
        let s_14_2: u32 = fn_state.offset_addr;
        // D s_14_3: call R_set(s_14_1, s_14_2)
        let s_14_3: () = R_set(state, tracer, s_14_1, s_14_2);
        // N s_14_4: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #2s : i
        let s_15_0: i128 = 2;
        // D s_15_1: read-var t:i64
        let s_15_1: i64 = fn_state.t;
        // D s_15_2: cast zx s_15_1 -> i
        let s_15_2: i128 = (i128::try_from(s_15_1).unwrap());
        // C s_15_3: const #0u : u8
        let s_15_3: bool = false;
        // C s_15_4: const #0u : u8
        let s_15_4: bool = false;
        // D s_15_5: call AArch32_SetLSInstructionSyndrome(s_15_0, s_15_3, s_15_2, s_15_4)
        let s_15_5: () = AArch32_SetLSInstructionSyndrome(
            state,
            tracer,
            s_15_0,
            s_15_3,
            s_15_2,
            s_15_4,
        );
        // N s_15_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #16993u : u32
        let s_16_0: u32 = 16993;
        // D s_16_1: read-reg s_16_0:u8
        let s_16_1: bool = {
            let value = state.read_register::<bool>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: cast zx s_16_1 -> bv
        let s_16_2: Bits = Bits::new(s_16_1 as u128, 1u16);
        // C s_16_3: const #1u : u8
        let s_16_3: bool = true;
        // C s_16_4: cast zx s_16_3 -> bv
        let s_16_4: Bits = Bits::new(s_16_3 as u128, 1u16);
        // D s_16_5: cmp-eq s_16_2 s_16_4
        let s_16_5: bool = ((s_16_2) == (s_16_4));
        // D s_16_6: write-var gs#302895 <= s_16_5
        fn_state.gs_302895 = s_16_5;
        // N s_16_7: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var n:i64
        let s_17_0: i64 = fn_state.n;
        // D s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_2: call R_read(s_17_1)
        let s_17_2: u32 = R_read(state, tracer, s_17_1);
        // D s_17_3: write-var address <= s_17_2
        fn_state.address = s_17_2;
        // N s_17_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var n:i64
        let s_18_0: i64 = fn_state.n;
        // D s_18_1: cast zx s_18_0 -> i
        let s_18_1: i128 = (i128::try_from(s_18_0).unwrap());
        // D s_18_2: call R_read(s_18_1)
        let s_18_2: u32 = R_read(state, tracer, s_18_1);
        // D s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 32u16);
        // D s_18_4: read-var offset:u32
        let s_18_4: u32 = fn_state.offset;
        // D s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 32u16);
        // D s_18_6: add s_18_3 s_18_5
        let s_18_6: Bits = (s_18_3 + s_18_5);
        // D s_18_7: cast reint s_18_6 -> u32
        let s_18_7: u32 = (s_18_6.value() as u32);
        // D s_18_8: write-var offset_addr <= s_18_7
        fn_state.offset_addr = s_18_7;
        // N s_18_9: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var m:i
        let s_19_0: i128 = fn_state.m;
        // D s_19_1: call R_read(s_19_0)
        let s_19_1: u32 = R_read(state, tracer, s_19_0);
        // D s_19_2: write-var offset <= s_19_1
        fn_state.offset = s_19_1;
        // N s_19_3: jump b3
        return block_3(state, tracer, fn_state);
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
}
