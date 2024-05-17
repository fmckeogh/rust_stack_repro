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
use PCStoreValue::*;
use neq_int::*;
use R_read::*;
use R_set::*;
use MemU_set::*;
use common::*;
pub fn execute_aarch32_instrs_STR_i_OpA_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    add: bool,
    imm32: u32,
    index: bool,
    n: i64,
    t: i64,
    wback: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_302940: bool,
        offset_addr: u32,
        ga_348226: u32,
        address: u32,
        add: bool,
        imm32: u32,
        index: bool,
        n: i64,
        t: i64,
        wback: bool,
    }
    let fn_state = FunctionState {
        add,
        imm32,
        index,
        n,
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
        // D s_0_0: read-var add:u8
        let s_0_0: bool = fn_state.add;
        // N s_0_1: branch s_0_0 b18 b1
        if s_0_0 {
            return block_18(state, tracer, fn_state);
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
        // D s_1_4: read-var imm32:u32
        let s_1_4: u32 = fn_state.imm32;
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
        // N s_2_1: branch s_2_0 b17 b3
        if s_2_0 {
            return block_17(state, tracer, fn_state);
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
        // N s_4_2: branch s_4_1 b16 b5
        if s_4_1 {
            return block_16(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#302940 <= s_5_0
        fn_state.gs_302940 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#302940:u8
        let s_6_0: bool = fn_state.gs_302940;
        // N s_6_1: branch s_6_0 b15 b7
        if s_6_0 {
            return block_15(state, tracer, fn_state);
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
        // C s_8_0: const #15s : i
        let s_8_0: i128 = 15;
        // D s_8_1: read-var t:i64
        let s_8_1: i64 = fn_state.t;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: cmp-eq s_8_2 s_8_0
        let s_8_3: bool = ((s_8_2) == (s_8_0));
        // N s_8_4: branch s_8_3 b14 b9
        if s_8_3 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var t:i64
        let s_9_0: i64 = fn_state.t;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: call R_read(s_9_1)
        let s_9_2: u32 = R_read(state, tracer, s_9_1);
        // D s_9_3: write-var ga#348226 <= s_9_2
        fn_state.ga_348226 = s_9_2;
        // N s_9_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #4s : i
        let s_10_0: i128 = 4;
        // D s_10_1: read-var ga#348226:u32
        let s_10_1: u32 = fn_state.ga_348226;
        // D s_10_2: cast zx s_10_1 -> bv
        let s_10_2: Bits = Bits::new(s_10_1 as u128, 32u16);
        // D s_10_3: read-var address:u32
        let s_10_3: u32 = fn_state.address;
        // D s_10_4: call MemU_set(s_10_3, s_10_0, s_10_2)
        let s_10_4: () = MemU_set(state, tracer, s_10_3, s_10_0, s_10_2);
        // N s_10_5: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var wback:u8
        let s_11_0: bool = fn_state.wback;
        // N s_11_1: branch s_11_0 b13 b12
        if s_11_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var n:i64
        let s_13_0: i64 = fn_state.n;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: read-var offset_addr:u32
        let s_13_2: u32 = fn_state.offset_addr;
        // D s_13_3: call R_set(s_13_1, s_13_2)
        let s_13_3: () = R_set(state, tracer, s_13_1, s_13_2);
        // N s_13_4: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call PCStoreValue(s_14_0)
        let s_14_1: u32 = PCStoreValue(state, tracer, s_14_0);
        // D s_14_2: write-var ga#348226 <= s_14_1
        fn_state.ga_348226 = s_14_1;
        // N s_14_3: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #4s : i
        let s_15_0: i128 = 4;
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
        // N s_15_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #15s : i
        let s_16_0: i128 = 15;
        // D s_16_1: read-var t:i64
        let s_16_1: i64 = fn_state.t;
        // D s_16_2: cast zx s_16_1 -> i
        let s_16_2: i128 = (i128::try_from(s_16_1).unwrap());
        // D s_16_3: call neq_int(s_16_2, s_16_0)
        let s_16_3: bool = neq_int(state, tracer, s_16_2, s_16_0);
        // D s_16_4: write-var gs#302940 <= s_16_3
        fn_state.gs_302940 = s_16_3;
        // N s_16_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var offset_addr:u32
        let s_17_0: u32 = fn_state.offset_addr;
        // D s_17_1: write-var address <= s_17_0
        fn_state.address = s_17_0;
        // N s_17_2: jump b4
        return block_4(state, tracer, fn_state);
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
        // D s_18_4: read-var imm32:u32
        let s_18_4: u32 = fn_state.imm32;
        // D s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 32u16);
        // D s_18_6: add s_18_3 s_18_5
        let s_18_6: Bits = (s_18_3 + s_18_5);
        // D s_18_7: cast reint s_18_6 -> u32
        let s_18_7: u32 = (s_18_6.value() as u32);
        // D s_18_8: write-var offset_addr <= s_18_7
        fn_state.offset_addr = s_18_7;
        // N s_18_9: jump b2
        return block_2(state, tracer, fn_state);
    }
}
