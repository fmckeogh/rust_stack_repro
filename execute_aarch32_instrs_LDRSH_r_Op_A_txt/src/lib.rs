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
use AArch32_SetLSInstructionSyndrome::*;
use neq_int::*;
use R_read::*;
use MemU_read::*;
use Shift::*;
use R_set::*;
use common::*;
pub fn execute_aarch32_instrs_LDRSH_r_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    add: bool,
    index: bool,
    m: i64,
    n: i64,
    shift_n: i128,
    shift_t: u32,
    t: i64,
    wback: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        offset: u32,
        gs_880949: Bits,
        data: u16,
        offset_addr: u32,
        gs_298136: bool,
        address: u32,
        add: bool,
        index: bool,
        m: i64,
        n: i64,
        shift_n: i128,
        shift_t: u32,
        t: i64,
        wback: bool,
    }
    let fn_state = FunctionState {
        add,
        index,
        m,
        n,
        shift_n,
        shift_t,
        t,
        wback,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var m:i64
        let s_0_0: i64 = fn_state.m;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // C s_0_3: const #16971u : u32
        let s_0_3: u32 = 16971;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: bool = {
            let value = state.read_register::<bool>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_2 -> bv
        let s_0_5: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_6: read-var shift_t:u32
        let s_0_6: u32 = fn_state.shift_t;
        // D s_0_7: read-var shift_n:i
        let s_0_7: i128 = fn_state.shift_n;
        // D s_0_8: call Shift(s_0_5, s_0_6, s_0_7, s_0_4)
        let s_0_8: Bits = Shift(state, tracer, s_0_5, s_0_6, s_0_7, s_0_4);
        // D s_0_9: cast reint s_0_8 -> u32
        let s_0_9: u32 = (s_0_8.value() as u32);
        // D s_0_10: write-var offset <= s_0_9
        fn_state.offset = s_0_9;
        // D s_0_11: read-var add:u8
        let s_0_11: bool = fn_state.add;
        // N s_0_12: branch s_0_11 b16 b1
        if s_0_11 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var n:i64
        let s_1_0: i64 = fn_state.n;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: call R_read(s_1_1)
        let s_1_2: u32 = R_read(state, tracer, s_1_1);
        // D s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 32u16);
        // D s_1_4: read-var offset:u32
        let s_1_4: u32 = fn_state.offset;
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 32u16);
        // D s_1_6: sub s_1_3 s_1_5
        let s_1_6: Bits = ((s_1_3) - (s_1_5));
        // D s_1_7: cast reint s_1_6 -> u32
        let s_1_7: u32 = (s_1_6.value() as u32);
        // D s_1_8: write-var offset_addr <= s_1_7
        fn_state.offset_addr = s_1_7;
        // N s_1_9: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var index:u8
        let s_2_0: bool = fn_state.index;
        // N s_2_1: branch s_2_0 b15 b3
        if s_2_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var n:i64
        let s_3_0: i64 = fn_state.n;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: call R_read(s_3_1)
        let s_3_2: u32 = R_read(state, tracer, s_3_1);
        // D s_3_3: write-var address <= s_3_2
        fn_state.address = s_3_2;
        // N s_3_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var wback:u8
        let s_4_0: bool = fn_state.wback;
        // D s_4_1: not s_4_0
        let s_4_1: bool = !s_4_0;
        // N s_4_2: branch s_4_1 b14 b5
        if s_4_1 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#298136 <= s_5_0
        fn_state.gs_298136 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#298136:u8
        let s_6_0: bool = fn_state.gs_298136;
        // N s_6_1: branch s_6_0 b13 b7
        if s_6_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #2s : i64
        let s_8_0: i64 = 2;
        // D s_8_1: read-var address:u32
        let s_8_1: u32 = fn_state.address;
        // D s_8_2: call MemU_read(s_8_1, s_8_0)
        let s_8_2: Bits = MemU_read(state, tracer, s_8_1, s_8_0);
        // D s_8_3: write-var gs#880949 <= s_8_2
        fn_state.gs_880949 = s_8_2;
        // N s_8_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#880949:bv
        let s_9_0: Bits = fn_state.gs_880949;
        // D s_9_1: cast reint s_9_0 -> u16
        let s_9_1: u16 = (s_9_0.value() as u16);
        // D s_9_2: write-var data <= s_9_1
        fn_state.data = s_9_1;
        // D s_9_3: read-var wback:u8
        let s_9_3: bool = fn_state.wback;
        // N s_9_4: branch s_9_3 b12 b10
        if s_9_3 {
            return block_12(state, tracer, fn_state);
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
        // C s_11_0: const #32s : i
        let s_11_0: i128 = 32;
        // D s_11_1: read-var data:u16
        let s_11_1: u16 = fn_state.data;
        // D s_11_2: cast zx s_11_1 -> bv
        let s_11_2: Bits = Bits::new(s_11_1 as u128, 16u16);
        // D s_11_3: bits-cast sx s_11_2 -> bv length s_11_0
        let s_11_3: Bits = s_11_2.sign_extend(s_11_0);
        // D s_11_4: cast reint s_11_3 -> u32
        let s_11_4: u32 = (s_11_3.value() as u32);
        // D s_11_5: read-var t:i64
        let s_11_5: i64 = fn_state.t;
        // D s_11_6: cast zx s_11_5 -> i
        let s_11_6: i128 = (i128::try_from(s_11_5).unwrap());
        // D s_11_7: call R_set(s_11_6, s_11_4)
        let s_11_7: () = R_set(state, tracer, s_11_6, s_11_4);
        // N s_11_8: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var n:i64
        let s_12_0: i64 = fn_state.n;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: read-var offset_addr:u32
        let s_12_2: u32 = fn_state.offset_addr;
        // D s_12_3: call R_set(s_12_1, s_12_2)
        let s_12_3: () = R_set(state, tracer, s_12_1, s_12_2);
        // N s_12_4: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #2s : i
        let s_13_0: i128 = 2;
        // D s_13_1: read-var t:i64
        let s_13_1: i64 = fn_state.t;
        // D s_13_2: cast zx s_13_1 -> i
        let s_13_2: i128 = (i128::try_from(s_13_1).unwrap());
        // C s_13_3: const #1u : u8
        let s_13_3: bool = true;
        // C s_13_4: const #0u : u8
        let s_13_4: bool = false;
        // D s_13_5: call AArch32_SetLSInstructionSyndrome(s_13_0, s_13_3, s_13_2, s_13_4)
        let s_13_5: () = AArch32_SetLSInstructionSyndrome(
            state,
            tracer,
            s_13_0,
            s_13_3,
            s_13_2,
            s_13_4,
        );
        // N s_13_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #15s : i
        let s_14_0: i128 = 15;
        // D s_14_1: read-var t:i64
        let s_14_1: i64 = fn_state.t;
        // D s_14_2: cast zx s_14_1 -> i
        let s_14_2: i128 = (i128::try_from(s_14_1).unwrap());
        // D s_14_3: call neq_int(s_14_2, s_14_0)
        let s_14_3: bool = neq_int(state, tracer, s_14_2, s_14_0);
        // D s_14_4: write-var gs#298136 <= s_14_3
        fn_state.gs_298136 = s_14_3;
        // N s_14_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var offset_addr:u32
        let s_15_0: u32 = fn_state.offset_addr;
        // D s_15_1: write-var address <= s_15_0
        fn_state.address = s_15_0;
        // N s_15_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var n:i64
        let s_16_0: i64 = fn_state.n;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: call R_read(s_16_1)
        let s_16_2: u32 = R_read(state, tracer, s_16_1);
        // D s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 32u16);
        // D s_16_4: read-var offset:u32
        let s_16_4: u32 = fn_state.offset;
        // D s_16_5: cast zx s_16_4 -> bv
        let s_16_5: Bits = Bits::new(s_16_4 as u128, 32u16);
        // D s_16_6: add s_16_3 s_16_5
        let s_16_6: Bits = (s_16_3 + s_16_5);
        // D s_16_7: cast reint s_16_6 -> u32
        let s_16_7: u32 = (s_16_6.value() as u32);
        // D s_16_8: write-var offset_addr <= s_16_7
        fn_state.offset_addr = s_16_7;
        // N s_16_9: jump b2
        return block_2(state, tracer, fn_state);
    }
}
