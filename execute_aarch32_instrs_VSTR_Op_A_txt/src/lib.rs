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
use BigEndian::*;
use CheckVFPEnabled::*;
use R_read::*;
use S_read::*;
use MemA_set::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VSTR_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    add: bool,
    d: i64,
    esize: i64,
    imm32: u32,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        address: u32,
        add: bool,
        d: i64,
        esize: i64,
        imm32: u32,
        n: i64,
    }
    let fn_state = FunctionState {
        add,
        d,
        esize,
        imm32,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #1u : u8
        let s_0_0: bool = true;
        // S s_0_1: call CheckVFPEnabled(s_0_0)
        let s_0_1: () = CheckVFPEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var add:u8
        let s_1_0: bool = fn_state.add;
        // N s_1_1: branch s_1_0 b14 b2
        if s_1_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var n:i64
        let s_2_0: i64 = fn_state.n;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call R_read(s_2_1)
        let s_2_2: u32 = R_read(state, tracer, s_2_1);
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 32u16);
        // D s_2_4: read-var imm32:u32
        let s_2_4: u32 = fn_state.imm32;
        // D s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 32u16);
        // D s_2_6: sub s_2_3 s_2_5
        let s_2_6: Bits = ((s_2_3) - (s_2_5));
        // D s_2_7: cast reint s_2_6 -> u32
        let s_2_7: u32 = (s_2_6.value() as u32);
        // D s_2_8: write-var address <= s_2_7
        fn_state.address = s_2_7;
        // N s_2_9: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var esize:i64
        let s_3_0: i64 = fn_state.esize;
        // C s_3_1: const #16s : i
        let s_3_1: i128 = 16;
        // D s_3_2: cast zx s_3_0 -> i
        let s_3_2: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_3: cmp-eq s_3_2 s_3_1
        let s_3_3: bool = ((s_3_2) == (s_3_1));
        // D s_3_4: not s_3_3
        let s_3_4: bool = !s_3_3;
        // N s_3_5: branch s_3_4 b5 b4
        if s_3_4 {
            return block_5(state, tracer, fn_state);
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
        // D s_4_2: call S_read(s_4_1)
        let s_4_2: u32 = S_read(state, tracer, s_4_1);
        // C s_4_3: const #0s : i
        let s_4_3: i128 = 0;
        // D s_4_4: cast zx s_4_2 -> bv
        let s_4_4: Bits = Bits::new(s_4_2 as u128, 32u16);
        // C s_4_5: const #1s : i64
        let s_4_5: i64 = 1;
        // C s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (i128::try_from(s_4_5).unwrap());
        // C s_4_7: const #15s : i
        let s_4_7: i128 = 15;
        // C s_4_8: add s_4_7 s_4_6
        let s_4_8: i128 = (s_4_7 + s_4_6);
        // D s_4_9: bit-extract s_4_4 s_4_3 s_4_8
        let s_4_9: Bits = (Bits::new(
            ((s_4_4) >> (s_4_3)).value(),
            u16::try_from(s_4_8).unwrap(),
        ));
        // D s_4_10: cast reint s_4_9 -> u16
        let s_4_10: u16 = (s_4_9.value() as u16);
        // C s_4_11: const #2s : i
        let s_4_11: i128 = 2;
        // D s_4_12: cast zx s_4_10 -> bv
        let s_4_12: Bits = Bits::new(s_4_10 as u128, 16u16);
        // D s_4_13: read-var address:u32
        let s_4_13: u32 = fn_state.address;
        // D s_4_14: call MemA_set(s_4_13, s_4_11, s_4_12)
        let s_4_14: () = MemA_set(state, tracer, s_4_13, s_4_11, s_4_12);
        // N s_4_15: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esize:i64
        let s_5_0: i64 = fn_state.esize;
        // C s_5_1: const #32s : i
        let s_5_1: i128 = 32;
        // D s_5_2: cast zx s_5_0 -> i
        let s_5_2: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_3: cmp-eq s_5_2 s_5_1
        let s_5_3: bool = ((s_5_2) == (s_5_1));
        // D s_5_4: not s_5_3
        let s_5_4: bool = !s_5_3;
        // N s_5_5: branch s_5_4 b7 b6
        if s_5_4 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var d:i64
        let s_6_0: i64 = fn_state.d;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: call S_read(s_6_1)
        let s_6_2: u32 = S_read(state, tracer, s_6_1);
        // C s_6_3: const #4s : i
        let s_6_3: i128 = 4;
        // D s_6_4: cast zx s_6_2 -> bv
        let s_6_4: Bits = Bits::new(s_6_2 as u128, 32u16);
        // D s_6_5: read-var address:u32
        let s_6_5: u32 = fn_state.address;
        // D s_6_6: call MemA_set(s_6_5, s_6_3, s_6_4)
        let s_6_6: () = MemA_set(state, tracer, s_6_5, s_6_3, s_6_4);
        // N s_6_7: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var esize:i64
        let s_7_0: i64 = fn_state.esize;
        // C s_7_1: const #64s : i
        let s_7_1: i128 = 64;
        // D s_7_2: cast zx s_7_0 -> i
        let s_7_2: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_3: cmp-eq s_7_2 s_7_1
        let s_7_3: bool = ((s_7_2) == (s_7_1));
        // D s_7_4: not s_7_3
        let s_7_4: bool = !s_7_3;
        // N s_7_5: branch s_7_4 b13 b8
        if s_7_4 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #2u : u32
        let s_8_0: u32 = 2;
        // S s_8_1: call BigEndian(s_8_0)
        let s_8_1: bool = BigEndian(state, tracer, s_8_0);
        // N s_8_2: branch s_8_1 b11 b9
        if s_8_1 {
            return block_11(state, tracer, fn_state);
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
        // D s_9_2: call D_read(s_9_1)
        let s_9_2: u64 = D_read(state, tracer, s_9_1);
        // C s_9_3: const #0s : i
        let s_9_3: i128 = 0;
        // D s_9_4: cast zx s_9_2 -> bv
        let s_9_4: Bits = Bits::new(s_9_2 as u128, 64u16);
        // C s_9_5: const #1s : i64
        let s_9_5: i64 = 1;
        // C s_9_6: cast zx s_9_5 -> i
        let s_9_6: i128 = (i128::try_from(s_9_5).unwrap());
        // C s_9_7: const #31s : i
        let s_9_7: i128 = 31;
        // C s_9_8: add s_9_7 s_9_6
        let s_9_8: i128 = (s_9_7 + s_9_6);
        // D s_9_9: bit-extract s_9_4 s_9_3 s_9_8
        let s_9_9: Bits = (Bits::new(
            ((s_9_4) >> (s_9_3)).value(),
            u16::try_from(s_9_8).unwrap(),
        ));
        // D s_9_10: cast reint s_9_9 -> u32
        let s_9_10: u32 = (s_9_9.value() as u32);
        // C s_9_11: const #4s : i
        let s_9_11: i128 = 4;
        // D s_9_12: cast zx s_9_10 -> bv
        let s_9_12: Bits = Bits::new(s_9_10 as u128, 32u16);
        // D s_9_13: read-var address:u32
        let s_9_13: u32 = fn_state.address;
        // D s_9_14: call MemA_set(s_9_13, s_9_11, s_9_12)
        let s_9_14: () = MemA_set(state, tracer, s_9_13, s_9_11, s_9_12);
        // N s_9_15: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #4s : i
        let s_10_0: i128 = 4;
        // D s_10_1: read-var address:u32
        let s_10_1: u32 = fn_state.address;
        // D s_10_2: cast zx s_10_1 -> bv
        let s_10_2: Bits = Bits::new(s_10_1 as u128, 32u16);
        // C s_10_3: cast cvt s_10_0 -> bv
        let s_10_3: Bits = Bits::new(s_10_0 as u128, 128);
        // D s_10_4: add s_10_2 s_10_3
        let s_10_4: Bits = (s_10_2 + s_10_3);
        // D s_10_5: cast reint s_10_4 -> u32
        let s_10_5: u32 = (s_10_4.value() as u32);
        // D s_10_6: read-var d:i64
        let s_10_6: i64 = fn_state.d;
        // D s_10_7: cast zx s_10_6 -> i
        let s_10_7: i128 = (i128::try_from(s_10_6).unwrap());
        // D s_10_8: call D_read(s_10_7)
        let s_10_8: u64 = D_read(state, tracer, s_10_7);
        // C s_10_9: const #32s : i
        let s_10_9: i128 = 32;
        // D s_10_10: cast zx s_10_8 -> bv
        let s_10_10: Bits = Bits::new(s_10_8 as u128, 64u16);
        // C s_10_11: const #1s : i64
        let s_10_11: i64 = 1;
        // C s_10_12: cast zx s_10_11 -> i
        let s_10_12: i128 = (i128::try_from(s_10_11).unwrap());
        // C s_10_13: const #31s : i
        let s_10_13: i128 = 31;
        // C s_10_14: add s_10_13 s_10_12
        let s_10_14: i128 = (s_10_13 + s_10_12);
        // D s_10_15: bit-extract s_10_10 s_10_9 s_10_14
        let s_10_15: Bits = (Bits::new(
            ((s_10_10) >> (s_10_9)).value(),
            u16::try_from(s_10_14).unwrap(),
        ));
        // D s_10_16: cast reint s_10_15 -> u32
        let s_10_16: u32 = (s_10_15.value() as u32);
        // C s_10_17: const #4s : i
        let s_10_17: i128 = 4;
        // D s_10_18: cast zx s_10_16 -> bv
        let s_10_18: Bits = Bits::new(s_10_16 as u128, 32u16);
        // D s_10_19: call MemA_set(s_10_5, s_10_17, s_10_18)
        let s_10_19: () = MemA_set(state, tracer, s_10_5, s_10_17, s_10_18);
        // N s_10_20: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var d:i64
        let s_11_0: i64 = fn_state.d;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: call D_read(s_11_1)
        let s_11_2: u64 = D_read(state, tracer, s_11_1);
        // C s_11_3: const #32s : i
        let s_11_3: i128 = 32;
        // D s_11_4: cast zx s_11_2 -> bv
        let s_11_4: Bits = Bits::new(s_11_2 as u128, 64u16);
        // C s_11_5: const #1s : i64
        let s_11_5: i64 = 1;
        // C s_11_6: cast zx s_11_5 -> i
        let s_11_6: i128 = (i128::try_from(s_11_5).unwrap());
        // C s_11_7: const #31s : i
        let s_11_7: i128 = 31;
        // C s_11_8: add s_11_7 s_11_6
        let s_11_8: i128 = (s_11_7 + s_11_6);
        // D s_11_9: bit-extract s_11_4 s_11_3 s_11_8
        let s_11_9: Bits = (Bits::new(
            ((s_11_4) >> (s_11_3)).value(),
            u16::try_from(s_11_8).unwrap(),
        ));
        // D s_11_10: cast reint s_11_9 -> u32
        let s_11_10: u32 = (s_11_9.value() as u32);
        // C s_11_11: const #4s : i
        let s_11_11: i128 = 4;
        // D s_11_12: cast zx s_11_10 -> bv
        let s_11_12: Bits = Bits::new(s_11_10 as u128, 32u16);
        // D s_11_13: read-var address:u32
        let s_11_13: u32 = fn_state.address;
        // D s_11_14: call MemA_set(s_11_13, s_11_11, s_11_12)
        let s_11_14: () = MemA_set(state, tracer, s_11_13, s_11_11, s_11_12);
        // N s_11_15: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #4s : i
        let s_12_0: i128 = 4;
        // D s_12_1: read-var address:u32
        let s_12_1: u32 = fn_state.address;
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 32u16);
        // C s_12_3: cast cvt s_12_0 -> bv
        let s_12_3: Bits = Bits::new(s_12_0 as u128, 128);
        // D s_12_4: add s_12_2 s_12_3
        let s_12_4: Bits = (s_12_2 + s_12_3);
        // D s_12_5: cast reint s_12_4 -> u32
        let s_12_5: u32 = (s_12_4.value() as u32);
        // D s_12_6: read-var d:i64
        let s_12_6: i64 = fn_state.d;
        // D s_12_7: cast zx s_12_6 -> i
        let s_12_7: i128 = (i128::try_from(s_12_6).unwrap());
        // D s_12_8: call D_read(s_12_7)
        let s_12_8: u64 = D_read(state, tracer, s_12_7);
        // C s_12_9: const #0s : i
        let s_12_9: i128 = 0;
        // D s_12_10: cast zx s_12_8 -> bv
        let s_12_10: Bits = Bits::new(s_12_8 as u128, 64u16);
        // C s_12_11: const #1s : i64
        let s_12_11: i64 = 1;
        // C s_12_12: cast zx s_12_11 -> i
        let s_12_12: i128 = (i128::try_from(s_12_11).unwrap());
        // C s_12_13: const #31s : i
        let s_12_13: i128 = 31;
        // C s_12_14: add s_12_13 s_12_12
        let s_12_14: i128 = (s_12_13 + s_12_12);
        // D s_12_15: bit-extract s_12_10 s_12_9 s_12_14
        let s_12_15: Bits = (Bits::new(
            ((s_12_10) >> (s_12_9)).value(),
            u16::try_from(s_12_14).unwrap(),
        ));
        // D s_12_16: cast reint s_12_15 -> u32
        let s_12_16: u32 = (s_12_15.value() as u32);
        // C s_12_17: const #4s : i
        let s_12_17: i128 = 4;
        // D s_12_18: cast zx s_12_16 -> bv
        let s_12_18: Bits = Bits::new(s_12_16 as u128, 32u16);
        // D s_12_19: call MemA_set(s_12_5, s_12_17, s_12_18)
        let s_12_19: () = MemA_set(state, tracer, s_12_5, s_12_17, s_12_18);
        // N s_12_20: return
        return;
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
        // D s_14_2: call R_read(s_14_1)
        let s_14_2: u32 = R_read(state, tracer, s_14_1);
        // D s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 32u16);
        // D s_14_4: read-var imm32:u32
        let s_14_4: u32 = fn_state.imm32;
        // D s_14_5: cast zx s_14_4 -> bv
        let s_14_5: Bits = Bits::new(s_14_4 as u128, 32u16);
        // D s_14_6: add s_14_3 s_14_5
        let s_14_6: Bits = (s_14_3 + s_14_5);
        // D s_14_7: cast reint s_14_6 -> u32
        let s_14_7: u32 = (s_14_6.value() as u32);
        // D s_14_8: write-var address <= s_14_7
        fn_state.address = s_14_7;
        // N s_14_9: jump b3
        return block_3(state, tracer, fn_state);
    }
}
