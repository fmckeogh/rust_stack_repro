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
use CreateAccDescLS64::*;
use CheckST64BV0Enabled::*;
use MemStore64BWithRet::*;
use BigEndian::*;
use X_read::*;
use AArch64_SetLSInstructionSyndrome::*;
use reverse_endianness::*;
use neq_int::*;
use X_set::*;
use SP_read::*;
use SPESampleGeneralPurposeLoadStore::*;
use CheckSPAlignment::*;
use SetLoadStoreType::*;
use SetIss2::*;
use common::*;
pub fn execute_aarch64_instrs_memory_atomicops_st_acc_st64bv0<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rs: u8,
    lst: u8,
    memop: u32,
    n: i64,
    s: i64,
    t: i64,
    tagchecked: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        address: u64,
        value_name: u64,
        data: u64,
        status: u64,
        accdesc: ProductType9878976b5bcce9c9,
        i: i64,
        Rs: u8,
        lst: u8,
        memop: u32,
        n: i64,
        s: i64,
        t: i64,
        tagchecked: bool,
    }
    let fn_state = FunctionState {
        Rs,
        lst,
        memop,
        n,
        s,
        t,
        tagchecked,
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
        // S s_0_1: call CheckST64BV0Enabled(s_0_0)
        let s_0_1: () = CheckST64BV0Enabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var memop:u32
        let s_1_0: u32 = fn_state.memop;
        // D s_1_1: read-var tagchecked:u8
        let s_1_1: bool = fn_state.tagchecked;
        // D s_1_2: call CreateAccDescLS64(s_1_0, s_1_1)
        let s_1_2: ProductType9878976b5bcce9c9 = CreateAccDescLS64(
            state,
            tracer,
            s_1_0,
            s_1_1,
        );
        // D s_1_3: write-var accdesc <= s_1_2
        fn_state.accdesc = s_1_2;
        // C s_1_4: const #64s : i64
        let s_1_4: i64 = 64;
        // D s_1_5: read-var t:i64
        let s_1_5: i64 = fn_state.t;
        // D s_1_6: cast zx s_1_5 -> i
        let s_1_6: i128 = (i128::try_from(s_1_5).unwrap());
        // D s_1_7: call X_read(s_1_6, s_1_4)
        let s_1_7: Bits = X_read(state, tracer, s_1_6, s_1_4);
        // D s_1_8: cast reint s_1_7 -> u64
        let s_1_8: u64 = (s_1_7.value() as u64);
        // C s_1_9: const #102464u : u32
        let s_1_9: u32 = 102464;
        // D s_1_10: read-reg s_1_9:u64
        let s_1_10: u64 = {
            let value = state.read_register::<u64>(s_1_9 as isize);
            tracer.read_register(s_1_9 as isize, value);
            value
        };
        // C s_1_11: const #0s : i
        let s_1_11: i128 = 0;
        // D s_1_12: cast zx s_1_10 -> bv
        let s_1_12: Bits = Bits::new(s_1_10 as u128, 64u16);
        // C s_1_13: const #1s : i64
        let s_1_13: i64 = 1;
        // C s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (i128::try_from(s_1_13).unwrap());
        // C s_1_15: const #31s : i
        let s_1_15: i128 = 31;
        // C s_1_16: add s_1_15 s_1_14
        let s_1_16: i128 = (s_1_15 + s_1_14);
        // D s_1_17: bit-extract s_1_12 s_1_11 s_1_16
        let s_1_17: Bits = (Bits::new(
            ((s_1_12) >> (s_1_11)).value(),
            u16::try_from(s_1_16).unwrap(),
        ));
        // D s_1_18: cast reint s_1_17 -> u32
        let s_1_18: u32 = (s_1_17.value() as u32);
        // C s_1_19: const #0s : i
        let s_1_19: i128 = 0;
        // D s_1_20: read-var value_name:u64
        let s_1_20: u64 = fn_state.value_name;
        // D s_1_21: cast zx s_1_20 -> bv
        let s_1_21: Bits = Bits::new(s_1_20 as u128, 64u16);
        // D s_1_22: cast zx s_1_18 -> bv
        let s_1_22: Bits = Bits::new(s_1_18 as u128, 32u16);
        // C s_1_23: const #31s : i
        let s_1_23: i128 = 31;
        // C s_1_24: const #1u : u64
        let s_1_24: u64 = 1;
        // C s_1_25: cast zx s_1_24 -> bv
        let s_1_25: Bits = Bits::new(s_1_24 as u128, 64u16);
        // C s_1_26: lsl s_1_25 s_1_23
        let s_1_26: Bits = s_1_25 << s_1_23;
        // C s_1_27: sub s_1_26 s_1_25
        let s_1_27: Bits = ((s_1_26) - (s_1_25));
        // D s_1_28: and s_1_22 s_1_27
        let s_1_28: Bits = ((s_1_22) & (s_1_27));
        // D s_1_29: lsl s_1_28 s_1_19
        let s_1_29: Bits = s_1_28 << s_1_19;
        // C s_1_30: lsl s_1_27 s_1_19
        let s_1_30: Bits = s_1_27 << s_1_19;
        // C s_1_31: cmpl s_1_30
        let s_1_31: Bits = !s_1_30;
        // D s_1_32: and s_1_21 s_1_31
        let s_1_32: Bits = ((s_1_21) & (s_1_31));
        // D s_1_33: or s_1_32 s_1_29
        let s_1_33: Bits = ((s_1_32) | (s_1_29));
        // D s_1_34: cast reint s_1_33 -> u64
        let s_1_34: u64 = (s_1_33.value() as u64);
        // D s_1_35: write-var value_name <= s_1_34
        fn_state.value_name = s_1_34;
        // C s_1_36: const #32s : i
        let s_1_36: i128 = 32;
        // D s_1_37: cast zx s_1_8 -> bv
        let s_1_37: Bits = Bits::new(s_1_8 as u128, 64u16);
        // C s_1_38: const #1s : i64
        let s_1_38: i64 = 1;
        // C s_1_39: cast zx s_1_38 -> i
        let s_1_39: i128 = (i128::try_from(s_1_38).unwrap());
        // C s_1_40: const #31s : i
        let s_1_40: i128 = 31;
        // C s_1_41: add s_1_40 s_1_39
        let s_1_41: i128 = (s_1_40 + s_1_39);
        // D s_1_42: bit-extract s_1_37 s_1_36 s_1_41
        let s_1_42: Bits = (Bits::new(
            ((s_1_37) >> (s_1_36)).value(),
            u16::try_from(s_1_41).unwrap(),
        ));
        // D s_1_43: cast reint s_1_42 -> u32
        let s_1_43: u32 = (s_1_42.value() as u32);
        // C s_1_44: const #32s : i
        let s_1_44: i128 = 32;
        // D s_1_45: read-var value_name:u64
        let s_1_45: u64 = fn_state.value_name;
        // D s_1_46: cast zx s_1_45 -> bv
        let s_1_46: Bits = Bits::new(s_1_45 as u128, 64u16);
        // D s_1_47: cast zx s_1_43 -> bv
        let s_1_47: Bits = Bits::new(s_1_43 as u128, 32u16);
        // C s_1_48: const #31s : i
        let s_1_48: i128 = 31;
        // C s_1_49: const #1u : u64
        let s_1_49: u64 = 1;
        // C s_1_50: cast zx s_1_49 -> bv
        let s_1_50: Bits = Bits::new(s_1_49 as u128, 64u16);
        // C s_1_51: lsl s_1_50 s_1_48
        let s_1_51: Bits = s_1_50 << s_1_48;
        // C s_1_52: sub s_1_51 s_1_50
        let s_1_52: Bits = ((s_1_51) - (s_1_50));
        // D s_1_53: and s_1_47 s_1_52
        let s_1_53: Bits = ((s_1_47) & (s_1_52));
        // D s_1_54: lsl s_1_53 s_1_44
        let s_1_54: Bits = s_1_53 << s_1_44;
        // C s_1_55: lsl s_1_52 s_1_44
        let s_1_55: Bits = s_1_52 << s_1_44;
        // C s_1_56: cmpl s_1_55
        let s_1_56: Bits = !s_1_55;
        // D s_1_57: and s_1_46 s_1_56
        let s_1_57: Bits = ((s_1_46) & (s_1_56));
        // D s_1_58: or s_1_57 s_1_54
        let s_1_58: Bits = ((s_1_57) | (s_1_54));
        // D s_1_59: cast reint s_1_58 -> u64
        let s_1_59: u64 = (s_1_58.value() as u64);
        // D s_1_60: write-var value_name <= s_1_59
        fn_state.value_name = s_1_59;
        // D s_1_61: read-var accdesc.1:struct
        let s_1_61: u32 = fn_state.accdesc._1;
        // D s_1_62: call BigEndian(s_1_61)
        let s_1_62: bool = BigEndian(state, tracer, s_1_61);
        // N s_1_63: branch s_1_62 b20 b2
        if s_1_62 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i
        let s_3_0: i128 = 0;
        // D s_3_1: read-var data:u512
        let s_3_1: u64 = fn_state.data;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 512u16);
        // D s_3_3: read-var value_name:u64
        let s_3_3: u64 = fn_state.value_name;
        // D s_3_4: cast zx s_3_3 -> bv
        let s_3_4: Bits = Bits::new(s_3_3 as u128, 64u16);
        // C s_3_5: const #63s : i
        let s_3_5: i128 = 63;
        // C s_3_6: const #1u : u64
        let s_3_6: u64 = 1;
        // C s_3_7: cast zx s_3_6 -> bv
        let s_3_7: Bits = Bits::new(s_3_6 as u128, 64u16);
        // C s_3_8: lsl s_3_7 s_3_5
        let s_3_8: Bits = s_3_7 << s_3_5;
        // C s_3_9: sub s_3_8 s_3_7
        let s_3_9: Bits = ((s_3_8) - (s_3_7));
        // D s_3_10: and s_3_4 s_3_9
        let s_3_10: Bits = ((s_3_4) & (s_3_9));
        // D s_3_11: lsl s_3_10 s_3_0
        let s_3_11: Bits = s_3_10 << s_3_0;
        // C s_3_12: lsl s_3_9 s_3_0
        let s_3_12: Bits = s_3_9 << s_3_0;
        // C s_3_13: cmpl s_3_12
        let s_3_13: Bits = !s_3_12;
        // D s_3_14: and s_3_2 s_3_13
        let s_3_14: Bits = ((s_3_2) & (s_3_13));
        // D s_3_15: or s_3_14 s_3_11
        let s_3_15: Bits = ((s_3_14) | (s_3_11));
        // D s_3_16: cast reint s_3_15 -> u512
        let s_3_16: u64 = (s_3_15.value() as u64);
        // D s_3_17: write-var data <= s_3_16
        fn_state.data = s_3_16;
        // C s_3_18: const #1s : i64
        let s_3_18: i64 = 1;
        // D s_3_19: write-var i <= s_3_18
        fn_state.i = s_3_18;
        // N s_3_20: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var i:i64
        let s_4_0: i64 = fn_state.i;
        // C s_4_1: const #7s : i64
        let s_4_1: i64 = 7;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b9 b5
        if s_4_2 {
            return block_9(state, tracer, fn_state);
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
        // D s_5_2: read-var i:i64
        let s_5_2: i64 = fn_state.i;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: add s_5_1 s_5_3
        let s_5_4: i128 = (s_5_1 + s_5_3);
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // C s_5_6: const #64s : i64
        let s_5_6: i64 = 64;
        // D s_5_7: cast zx s_5_5 -> i
        let s_5_7: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_8: call X_read(s_5_7, s_5_6)
        let s_5_8: Bits = X_read(state, tracer, s_5_7, s_5_6);
        // D s_5_9: cast reint s_5_8 -> u64
        let s_5_9: u64 = (s_5_8.value() as u64);
        // D s_5_10: write-var value_name <= s_5_9
        fn_state.value_name = s_5_9;
        // D s_5_11: read-var accdesc.1:struct
        let s_5_11: u32 = fn_state.accdesc._1;
        // D s_5_12: call BigEndian(s_5_11)
        let s_5_12: bool = BigEndian(state, tracer, s_5_11);
        // N s_5_13: branch s_5_12 b8 b6
        if s_5_12 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #64s : i
        let s_7_0: i128 = 64;
        // D s_7_1: read-var i:i64
        let s_7_1: i64 = fn_state.i;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: mul s_7_0 s_7_2
        let s_7_3: i128 = ((s_7_0) * (s_7_2));
        // D s_7_4: cast reint s_7_3 -> i64
        let s_7_4: i64 = (s_7_3 as i64);
        // C s_7_5: const #63s : i
        let s_7_5: i128 = 63;
        // D s_7_6: cast zx s_7_4 -> i
        let s_7_6: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_7: add s_7_5 s_7_6
        let s_7_7: i128 = (s_7_5 + s_7_6);
        // D s_7_8: cast reint s_7_7 -> i64
        let s_7_8: i64 = (s_7_7 as i64);
        // C s_7_9: const #64s : i
        let s_7_9: i128 = 64;
        // D s_7_10: read-var i:i64
        let s_7_10: i64 = fn_state.i;
        // D s_7_11: cast zx s_7_10 -> i
        let s_7_11: i128 = (i128::try_from(s_7_10).unwrap());
        // D s_7_12: mul s_7_9 s_7_11
        let s_7_12: i128 = ((s_7_9) * (s_7_11));
        // D s_7_13: cast reint s_7_12 -> i64
        let s_7_13: i64 = (s_7_12 as i64);
        // D s_7_14: read-var data:u512
        let s_7_14: u64 = fn_state.data;
        // D s_7_15: cast zx s_7_14 -> bv
        let s_7_15: Bits = Bits::new(s_7_14 as u128, 512u16);
        // D s_7_16: cast zx s_7_8 -> i
        let s_7_16: i128 = (i128::try_from(s_7_8).unwrap());
        // D s_7_17: cast zx s_7_13 -> i
        let s_7_17: i128 = (i128::try_from(s_7_13).unwrap());
        // D s_7_18: read-var value_name:u64
        let s_7_18: u64 = fn_state.value_name;
        // D s_7_19: cast zx s_7_18 -> bv
        let s_7_19: Bits = Bits::new(s_7_18 as u128, 64u16);
        // D s_7_20: sub s_7_16 s_7_17
        let s_7_20: i128 = ((s_7_16) - (s_7_17));
        // C s_7_21: const #1u : u64
        let s_7_21: u64 = 1;
        // C s_7_22: cast zx s_7_21 -> bv
        let s_7_22: Bits = Bits::new(s_7_21 as u128, 64u16);
        // D s_7_23: lsl s_7_22 s_7_20
        let s_7_23: Bits = s_7_22 << s_7_20;
        // D s_7_24: sub s_7_23 s_7_22
        let s_7_24: Bits = ((s_7_23) - (s_7_22));
        // D s_7_25: and s_7_19 s_7_24
        let s_7_25: Bits = ((s_7_19) & (s_7_24));
        // D s_7_26: lsl s_7_25 s_7_17
        let s_7_26: Bits = s_7_25 << s_7_17;
        // D s_7_27: lsl s_7_24 s_7_17
        let s_7_27: Bits = s_7_24 << s_7_17;
        // D s_7_28: cmpl s_7_27
        let s_7_28: Bits = !s_7_27;
        // D s_7_29: and s_7_15 s_7_28
        let s_7_29: Bits = ((s_7_15) & (s_7_28));
        // D s_7_30: or s_7_29 s_7_26
        let s_7_30: Bits = ((s_7_29) | (s_7_26));
        // D s_7_31: cast reint s_7_30 -> u512
        let s_7_31: u64 = (s_7_30.value() as u64);
        // D s_7_32: write-var data <= s_7_31
        fn_state.data = s_7_31;
        // D s_7_33: read-var i:i64
        let s_7_33: i64 = fn_state.i;
        // C s_7_34: const #1s : i64
        let s_7_34: i64 = 1;
        // D s_7_35: add s_7_33 s_7_34
        let s_7_35: i64 = (s_7_33 + s_7_34);
        // D s_7_36: write-var i <= s_7_35
        fn_state.i = s_7_35;
        // N s_7_37: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var value_name:u64
        let s_8_0: u64 = fn_state.value_name;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 64u16);
        // D s_8_2: call reverse_endianness(s_8_1)
        let s_8_2: Bits = reverse_endianness(state, tracer, s_8_1);
        // D s_8_3: cast reint s_8_2 -> u64
        let s_8_3: u64 = (s_8_2.value() as u64);
        // D s_8_4: write-var value_name <= s_8_3
        fn_state.value_name = s_8_3;
        // N s_8_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #31s : i
        let s_9_0: i128 = 31;
        // D s_9_1: read-var n:i64
        let s_9_1: i64 = fn_state.n;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: cmp-eq s_9_2 s_9_0
        let s_9_3: bool = ((s_9_2) == (s_9_0));
        // N s_9_4: branch s_9_3 b18 b10
        if s_9_3 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #64s : i64
        let s_10_0: i64 = 64;
        // D s_10_1: read-var n:i64
        let s_10_1: i64 = fn_state.n;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: call X_read(s_10_2, s_10_0)
        let s_10_3: Bits = X_read(state, tracer, s_10_2, s_10_0);
        // D s_10_4: cast reint s_10_3 -> u64
        let s_10_4: u64 = (s_10_3.value() as u64);
        // D s_10_5: write-var address <= s_10_4
        fn_state.address = s_10_4;
        // N s_10_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var lst:u8
        let s_11_0: u8 = fn_state.lst;
        // D s_11_1: call SetLoadStoreType(s_11_0)
        let s_11_1: () = SetLoadStoreType(state, tracer, s_11_0);
        // C s_11_2: const #64s : i
        let s_11_2: i128 = 64;
        // D s_11_3: read-var t:i64
        let s_11_3: i64 = fn_state.t;
        // D s_11_4: cast zx s_11_3 -> i
        let s_11_4: i128 = (i128::try_from(s_11_3).unwrap());
        // C s_11_5: const #0u : u8
        let s_11_5: bool = false;
        // C s_11_6: const #1u : u8
        let s_11_6: bool = true;
        // C s_11_7: const #0u : u8
        let s_11_7: bool = false;
        // D s_11_8: call AArch64_SetLSInstructionSyndrome(s_11_2, s_11_5, s_11_4, s_11_6, s_11_7)
        let s_11_8: () = AArch64_SetLSInstructionSyndrome(
            state,
            tracer,
            s_11_2,
            s_11_5,
            s_11_4,
            s_11_6,
            s_11_7,
        );
        // D s_11_9: read-var Rs:u8
        let s_11_9: u8 = fn_state.Rs;
        // D s_11_10: call SetIss2(s_11_9)
        let s_11_10: () = SetIss2(state, tracer, s_11_9);
        // D s_11_11: read-var address:u64
        let s_11_11: u64 = fn_state.address;
        // D s_11_12: read-var data:u512
        let s_11_12: u64 = fn_state.data;
        // D s_11_13: read-var accdesc:struct
        let s_11_13: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_11_14: call MemStore64BWithRet(s_11_11, s_11_12, s_11_13)
        let s_11_14: u64 = MemStore64BWithRet(state, tracer, s_11_11, s_11_12, s_11_13);
        // D s_11_15: write-var status <= s_11_14
        fn_state.status = s_11_14;
        // N s_11_16: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #31s : i
        let s_12_0: i128 = 31;
        // D s_12_1: read-var s:i64
        let s_12_1: i64 = fn_state.s;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: call neq_int(s_12_2, s_12_0)
        let s_12_3: bool = neq_int(state, tracer, s_12_2, s_12_0);
        // N s_12_4: branch s_12_3 b17 b13
        if s_12_3 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #22416u : u32
        let s_14_0: u32 = 22416;
        // D s_14_1: read-reg s_14_0:u8
        let s_14_1: bool = {
            let value = state.read_register::<bool>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // N s_14_2: branch s_14_1 b16 b15
        if s_14_1 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call SPESampleGeneralPurposeLoadStore(s_16_0)
        let s_16_1: () = SPESampleGeneralPurposeLoadStore(state, tracer, s_16_0);
        // N s_16_2: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #64s : i64
        let s_17_0: i64 = 64;
        // D s_17_1: read-var s:i64
        let s_17_1: i64 = fn_state.s;
        // D s_17_2: cast zx s_17_1 -> i
        let s_17_2: i128 = (i128::try_from(s_17_1).unwrap());
        // D s_17_3: read-var status:u64
        let s_17_3: u64 = fn_state.status;
        // D s_17_4: cast zx s_17_3 -> bv
        let s_17_4: Bits = Bits::new(s_17_3 as u128, 64u16);
        // D s_17_5: call X_set(s_17_2, s_17_0, s_17_4)
        let s_17_5: () = X_set(state, tracer, s_17_2, s_17_0, s_17_4);
        // N s_17_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call CheckSPAlignment(s_18_0)
        let s_18_1: () = CheckSPAlignment(state, tracer, s_18_0);
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call SP_read(s_19_0)
        let s_19_1: u64 = SP_read(state, tracer, s_19_0);
        // D s_19_2: write-var address <= s_19_1
        fn_state.address = s_19_1;
        // N s_19_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var value_name:u64
        let s_20_0: u64 = fn_state.value_name;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 64u16);
        // D s_20_2: call reverse_endianness(s_20_1)
        let s_20_2: Bits = reverse_endianness(state, tracer, s_20_1);
        // D s_20_3: cast reint s_20_2 -> u64
        let s_20_3: u64 = (s_20_2.value() as u64);
        // D s_20_4: write-var value_name <= s_20_3
        fn_state.value_name = s_20_3;
        // N s_20_5: jump b3
        return block_3(state, tracer, fn_state);
    }
}
