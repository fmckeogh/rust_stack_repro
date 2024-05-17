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
use IsAligned__1::*;
use R_read::*;
use R_set::*;
use MemA_set::*;
use common::*;
pub fn execute_aarch32_instrs_STRD_r_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    add: bool,
    index: bool,
    m: i64,
    n: i64,
    t: i64,
    t2: i64,
    wback: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        data: u64,
        address: u32,
        offset_addr: u32,
        add: bool,
        index: bool,
        m: i64,
        n: i64,
        t: i64,
        t2: i64,
        wback: bool,
    }
    let fn_state = FunctionState {
        add,
        index,
        m,
        n,
        t,
        t2,
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
        // N s_0_1: branch s_0_0 b15 b1
        if s_0_0 {
            return block_15(state, tracer, fn_state);
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
        // D s_1_3: read-var m:i64
        let s_1_3: i64 = fn_state.m;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: call R_read(s_1_4)
        let s_1_5: u32 = R_read(state, tracer, s_1_4);
        // D s_1_6: cast zx s_1_2 -> bv
        let s_1_6: Bits = Bits::new(s_1_2 as u128, 32u16);
        // D s_1_7: cast zx s_1_5 -> bv
        let s_1_7: Bits = Bits::new(s_1_5 as u128, 32u16);
        // D s_1_8: sub s_1_6 s_1_7
        let s_1_8: Bits = ((s_1_6) - (s_1_7));
        // D s_1_9: cast reint s_1_8 -> u32
        let s_1_9: u32 = (s_1_8.value() as u32);
        // D s_1_10: write-var offset_addr <= s_1_9
        fn_state.offset_addr = s_1_9;
        // N s_1_11: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var index:u8
        let s_2_0: bool = fn_state.index;
        // N s_2_1: branch s_2_0 b14 b3
        if s_2_0 {
            return block_14(state, tracer, fn_state);
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
        // C s_4_0: const #8s : i
        let s_4_0: i128 = 8;
        // D s_4_1: read-var address:u32
        let s_4_1: u32 = fn_state.address;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 32u16);
        // D s_4_3: call IsAligned__1(s_4_2, s_4_0)
        let s_4_3: bool = IsAligned__1(state, tracer, s_4_2, s_4_0);
        // N s_4_4: branch s_4_3 b10 b5
        if s_4_3 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var t:i64
        let s_5_0: i64 = fn_state.t;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: call R_read(s_5_1)
        let s_5_2: u32 = R_read(state, tracer, s_5_1);
        // C s_5_3: const #4s : i
        let s_5_3: i128 = 4;
        // D s_5_4: cast zx s_5_2 -> bv
        let s_5_4: Bits = Bits::new(s_5_2 as u128, 32u16);
        // D s_5_5: read-var address:u32
        let s_5_5: u32 = fn_state.address;
        // D s_5_6: call MemA_set(s_5_5, s_5_3, s_5_4)
        let s_5_6: () = MemA_set(state, tracer, s_5_5, s_5_3, s_5_4);
        // N s_5_7: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #4s : i
        let s_6_0: i128 = 4;
        // D s_6_1: read-var address:u32
        let s_6_1: u32 = fn_state.address;
        // D s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 32u16);
        // C s_6_3: cast cvt s_6_0 -> bv
        let s_6_3: Bits = Bits::new(s_6_0 as u128, 128);
        // D s_6_4: add s_6_2 s_6_3
        let s_6_4: Bits = (s_6_2 + s_6_3);
        // D s_6_5: cast reint s_6_4 -> u32
        let s_6_5: u32 = (s_6_4.value() as u32);
        // D s_6_6: read-var t2:i64
        let s_6_6: i64 = fn_state.t2;
        // D s_6_7: cast zx s_6_6 -> i
        let s_6_7: i128 = (i128::try_from(s_6_6).unwrap());
        // D s_6_8: call R_read(s_6_7)
        let s_6_8: u32 = R_read(state, tracer, s_6_7);
        // C s_6_9: const #4s : i
        let s_6_9: i128 = 4;
        // D s_6_10: cast zx s_6_8 -> bv
        let s_6_10: Bits = Bits::new(s_6_8 as u128, 32u16);
        // D s_6_11: call MemA_set(s_6_5, s_6_9, s_6_10)
        let s_6_11: () = MemA_set(state, tracer, s_6_5, s_6_9, s_6_10);
        // N s_6_12: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var wback:u8
        let s_7_0: bool = fn_state.wback;
        // N s_7_1: branch s_7_0 b9 b8
        if s_7_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: return
        return;
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
        // D s_9_2: read-var offset_addr:u32
        let s_9_2: u32 = fn_state.offset_addr;
        // D s_9_3: call R_set(s_9_1, s_9_2)
        let s_9_3: () = R_set(state, tracer, s_9_1, s_9_2);
        // N s_9_4: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1u : u32
        let s_10_0: u32 = 1;
        // S s_10_1: call BigEndian(s_10_0)
        let s_10_1: bool = BigEndian(state, tracer, s_10_0);
        // N s_10_2: branch s_10_1 b13 b11
        if s_10_1 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
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
        // D s_11_4: read-var data:u64
        let s_11_4: u64 = fn_state.data;
        // D s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 64u16);
        // D s_11_6: cast zx s_11_2 -> bv
        let s_11_6: Bits = Bits::new(s_11_2 as u128, 32u16);
        // C s_11_7: const #31s : i
        let s_11_7: i128 = 31;
        // C s_11_8: const #1u : u64
        let s_11_8: u64 = 1;
        // C s_11_9: cast zx s_11_8 -> bv
        let s_11_9: Bits = Bits::new(s_11_8 as u128, 64u16);
        // C s_11_10: lsl s_11_9 s_11_7
        let s_11_10: Bits = s_11_9 << s_11_7;
        // C s_11_11: sub s_11_10 s_11_9
        let s_11_11: Bits = ((s_11_10) - (s_11_9));
        // D s_11_12: and s_11_6 s_11_11
        let s_11_12: Bits = ((s_11_6) & (s_11_11));
        // D s_11_13: lsl s_11_12 s_11_3
        let s_11_13: Bits = s_11_12 << s_11_3;
        // C s_11_14: lsl s_11_11 s_11_3
        let s_11_14: Bits = s_11_11 << s_11_3;
        // C s_11_15: cmpl s_11_14
        let s_11_15: Bits = !s_11_14;
        // D s_11_16: and s_11_5 s_11_15
        let s_11_16: Bits = ((s_11_5) & (s_11_15));
        // D s_11_17: or s_11_16 s_11_13
        let s_11_17: Bits = ((s_11_16) | (s_11_13));
        // D s_11_18: cast reint s_11_17 -> u64
        let s_11_18: u64 = (s_11_17.value() as u64);
        // D s_11_19: write-var data <= s_11_18
        fn_state.data = s_11_18;
        // D s_11_20: read-var t2:i64
        let s_11_20: i64 = fn_state.t2;
        // D s_11_21: cast zx s_11_20 -> i
        let s_11_21: i128 = (i128::try_from(s_11_20).unwrap());
        // D s_11_22: call R_read(s_11_21)
        let s_11_22: u32 = R_read(state, tracer, s_11_21);
        // C s_11_23: const #32s : i
        let s_11_23: i128 = 32;
        // D s_11_24: read-var data:u64
        let s_11_24: u64 = fn_state.data;
        // D s_11_25: cast zx s_11_24 -> bv
        let s_11_25: Bits = Bits::new(s_11_24 as u128, 64u16);
        // D s_11_26: cast zx s_11_22 -> bv
        let s_11_26: Bits = Bits::new(s_11_22 as u128, 32u16);
        // C s_11_27: const #31s : i
        let s_11_27: i128 = 31;
        // C s_11_28: const #1u : u64
        let s_11_28: u64 = 1;
        // C s_11_29: cast zx s_11_28 -> bv
        let s_11_29: Bits = Bits::new(s_11_28 as u128, 64u16);
        // C s_11_30: lsl s_11_29 s_11_27
        let s_11_30: Bits = s_11_29 << s_11_27;
        // C s_11_31: sub s_11_30 s_11_29
        let s_11_31: Bits = ((s_11_30) - (s_11_29));
        // D s_11_32: and s_11_26 s_11_31
        let s_11_32: Bits = ((s_11_26) & (s_11_31));
        // D s_11_33: lsl s_11_32 s_11_23
        let s_11_33: Bits = s_11_32 << s_11_23;
        // C s_11_34: lsl s_11_31 s_11_23
        let s_11_34: Bits = s_11_31 << s_11_23;
        // C s_11_35: cmpl s_11_34
        let s_11_35: Bits = !s_11_34;
        // D s_11_36: and s_11_25 s_11_35
        let s_11_36: Bits = ((s_11_25) & (s_11_35));
        // D s_11_37: or s_11_36 s_11_33
        let s_11_37: Bits = ((s_11_36) | (s_11_33));
        // D s_11_38: cast reint s_11_37 -> u64
        let s_11_38: u64 = (s_11_37.value() as u64);
        // D s_11_39: write-var data <= s_11_38
        fn_state.data = s_11_38;
        // N s_11_40: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #8s : i
        let s_12_0: i128 = 8;
        // D s_12_1: read-var data:u64
        let s_12_1: u64 = fn_state.data;
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 64u16);
        // D s_12_3: read-var address:u32
        let s_12_3: u32 = fn_state.address;
        // D s_12_4: call MemA_set(s_12_3, s_12_0, s_12_2)
        let s_12_4: () = MemA_set(state, tracer, s_12_3, s_12_0, s_12_2);
        // N s_12_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var t:i64
        let s_13_0: i64 = fn_state.t;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: call R_read(s_13_1)
        let s_13_2: u32 = R_read(state, tracer, s_13_1);
        // C s_13_3: const #32s : i
        let s_13_3: i128 = 32;
        // D s_13_4: read-var data:u64
        let s_13_4: u64 = fn_state.data;
        // D s_13_5: cast zx s_13_4 -> bv
        let s_13_5: Bits = Bits::new(s_13_4 as u128, 64u16);
        // D s_13_6: cast zx s_13_2 -> bv
        let s_13_6: Bits = Bits::new(s_13_2 as u128, 32u16);
        // C s_13_7: const #31s : i
        let s_13_7: i128 = 31;
        // C s_13_8: const #1u : u64
        let s_13_8: u64 = 1;
        // C s_13_9: cast zx s_13_8 -> bv
        let s_13_9: Bits = Bits::new(s_13_8 as u128, 64u16);
        // C s_13_10: lsl s_13_9 s_13_7
        let s_13_10: Bits = s_13_9 << s_13_7;
        // C s_13_11: sub s_13_10 s_13_9
        let s_13_11: Bits = ((s_13_10) - (s_13_9));
        // D s_13_12: and s_13_6 s_13_11
        let s_13_12: Bits = ((s_13_6) & (s_13_11));
        // D s_13_13: lsl s_13_12 s_13_3
        let s_13_13: Bits = s_13_12 << s_13_3;
        // C s_13_14: lsl s_13_11 s_13_3
        let s_13_14: Bits = s_13_11 << s_13_3;
        // C s_13_15: cmpl s_13_14
        let s_13_15: Bits = !s_13_14;
        // D s_13_16: and s_13_5 s_13_15
        let s_13_16: Bits = ((s_13_5) & (s_13_15));
        // D s_13_17: or s_13_16 s_13_13
        let s_13_17: Bits = ((s_13_16) | (s_13_13));
        // D s_13_18: cast reint s_13_17 -> u64
        let s_13_18: u64 = (s_13_17.value() as u64);
        // D s_13_19: write-var data <= s_13_18
        fn_state.data = s_13_18;
        // D s_13_20: read-var t2:i64
        let s_13_20: i64 = fn_state.t2;
        // D s_13_21: cast zx s_13_20 -> i
        let s_13_21: i128 = (i128::try_from(s_13_20).unwrap());
        // D s_13_22: call R_read(s_13_21)
        let s_13_22: u32 = R_read(state, tracer, s_13_21);
        // C s_13_23: const #0s : i
        let s_13_23: i128 = 0;
        // D s_13_24: read-var data:u64
        let s_13_24: u64 = fn_state.data;
        // D s_13_25: cast zx s_13_24 -> bv
        let s_13_25: Bits = Bits::new(s_13_24 as u128, 64u16);
        // D s_13_26: cast zx s_13_22 -> bv
        let s_13_26: Bits = Bits::new(s_13_22 as u128, 32u16);
        // C s_13_27: const #31s : i
        let s_13_27: i128 = 31;
        // C s_13_28: const #1u : u64
        let s_13_28: u64 = 1;
        // C s_13_29: cast zx s_13_28 -> bv
        let s_13_29: Bits = Bits::new(s_13_28 as u128, 64u16);
        // C s_13_30: lsl s_13_29 s_13_27
        let s_13_30: Bits = s_13_29 << s_13_27;
        // C s_13_31: sub s_13_30 s_13_29
        let s_13_31: Bits = ((s_13_30) - (s_13_29));
        // D s_13_32: and s_13_26 s_13_31
        let s_13_32: Bits = ((s_13_26) & (s_13_31));
        // D s_13_33: lsl s_13_32 s_13_23
        let s_13_33: Bits = s_13_32 << s_13_23;
        // C s_13_34: lsl s_13_31 s_13_23
        let s_13_34: Bits = s_13_31 << s_13_23;
        // C s_13_35: cmpl s_13_34
        let s_13_35: Bits = !s_13_34;
        // D s_13_36: and s_13_25 s_13_35
        let s_13_36: Bits = ((s_13_25) & (s_13_35));
        // D s_13_37: or s_13_36 s_13_33
        let s_13_37: Bits = ((s_13_36) | (s_13_33));
        // D s_13_38: cast reint s_13_37 -> u64
        let s_13_38: u64 = (s_13_37.value() as u64);
        // D s_13_39: write-var data <= s_13_38
        fn_state.data = s_13_38;
        // N s_13_40: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var offset_addr:u32
        let s_14_0: u32 = fn_state.offset_addr;
        // D s_14_1: write-var address <= s_14_0
        fn_state.address = s_14_0;
        // N s_14_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var n:i64
        let s_15_0: i64 = fn_state.n;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: call R_read(s_15_1)
        let s_15_2: u32 = R_read(state, tracer, s_15_1);
        // D s_15_3: read-var m:i64
        let s_15_3: i64 = fn_state.m;
        // D s_15_4: cast zx s_15_3 -> i
        let s_15_4: i128 = (i128::try_from(s_15_3).unwrap());
        // D s_15_5: call R_read(s_15_4)
        let s_15_5: u32 = R_read(state, tracer, s_15_4);
        // D s_15_6: cast zx s_15_2 -> bv
        let s_15_6: Bits = Bits::new(s_15_2 as u128, 32u16);
        // D s_15_7: cast zx s_15_5 -> bv
        let s_15_7: Bits = Bits::new(s_15_5 as u128, 32u16);
        // D s_15_8: add s_15_6 s_15_7
        let s_15_8: Bits = (s_15_6 + s_15_7);
        // D s_15_9: cast reint s_15_8 -> u32
        let s_15_9: u32 = (s_15_8.value() as u32);
        // D s_15_10: write-var offset_addr <= s_15_9
        fn_state.offset_addr = s_15_9;
        // N s_15_11: jump b2
        return block_2(state, tracer, fn_state);
    }
}
