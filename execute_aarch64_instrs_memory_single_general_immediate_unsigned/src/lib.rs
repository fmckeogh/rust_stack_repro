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
use X_set::*;
use u__UNKNOWN_bits::*;
use u__id::*;
use SP_set::*;
use Mem_read::*;
use SPESampleGeneralPurposeLoadStore::*;
use integer_subrange::*;
use SP_read::*;
use CreateAccDescGPR::*;
use X_read::*;
use AArch64_SetLSInstructionSyndrome::*;
use Prefetch::*;
use CheckSPAlignment::*;
use Mem_set::*;
use common::*;
pub fn execute_aarch64_instrs_memory_single_general_immediate_unsigned<T: Tracer>(
    state: &mut State,
    tracer: &T,
    datasize: i64,
    memop: u32,
    n: i64,
    nontemporal: bool,
    offset: u64,
    postindex: bool,
    regsize: i64,
    rt_unknown: bool,
    is_signed: bool,
    t: i64,
    tagchecked: bool,
    wb_unknown: bool,
    wback: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_165742: bool,
        address: u64,
        regsizeshadow_1780: i64,
        datasizeshadow_1781: i64,
        datashadow_1782: Bits,
        data: Bits,
        gs_165739: bool,
        accdesc: ProductType9878976b5bcce9c9,
        datasize: i64,
        memop: u32,
        n: i64,
        nontemporal: bool,
        offset: u64,
        postindex: bool,
        regsize: i64,
        rt_unknown: bool,
        is_signed: bool,
        t: i64,
        tagchecked: bool,
        wb_unknown: bool,
        wback: bool,
    }
    let fn_state = FunctionState {
        datasize,
        memop,
        n,
        nontemporal,
        offset,
        postindex,
        regsize,
        rt_unknown,
        is_signed,
        t,
        tagchecked,
        wb_unknown,
        wback,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var regsize:i64
        let s_0_0: i64 = fn_state.regsize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var regsizeshadow#1780 <= s_0_2
        fn_state.regsizeshadow_1780 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1781 <= s_0_6
        fn_state.datasizeshadow_1781 = s_0_6;
        // C s_0_8: const #16975u : u32
        let s_0_8: u32 = 16975;
        // D s_0_9: read-reg s_0_8:u8
        let s_0_9: u8 = {
            let value = state.read_register::<u8>(s_0_8 as isize);
            tracer.read_register(s_0_8 as isize, value);
            value
        };
        // D s_0_10: cast zx s_0_9 -> bv
        let s_0_10: Bits = Bits::new(s_0_9 as u128, 2u16);
        // C s_0_11: const #448u : u32
        let s_0_11: u32 = 448;
        // D s_0_12: read-reg s_0_11:u8
        let s_0_12: u8 = {
            let value = state.read_register::<u8>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 2u16);
        // D s_0_14: cmp-ne s_0_10 s_0_13
        let s_0_14: bool = ((s_0_10) != (s_0_13));
        // D s_0_15: read-var memop:u32
        let s_0_15: u32 = fn_state.memop;
        // D s_0_16: read-var nontemporal:u8
        let s_0_16: bool = fn_state.nontemporal;
        // D s_0_17: read-var tagchecked:u8
        let s_0_17: bool = fn_state.tagchecked;
        // D s_0_18: call CreateAccDescGPR(s_0_15, s_0_16, s_0_14, s_0_17)
        let s_0_18: ProductType9878976b5bcce9c9 = CreateAccDescGPR(
            state,
            tracer,
            s_0_15,
            s_0_16,
            s_0_14,
            s_0_17,
        );
        // D s_0_19: write-var accdesc <= s_0_18
        fn_state.accdesc = s_0_18;
        // C s_0_20: const #31s : i
        let s_0_20: i128 = 31;
        // D s_0_21: read-var n:i64
        let s_0_21: i64 = fn_state.n;
        // D s_0_22: cast zx s_0_21 -> i
        let s_0_22: i128 = (i128::try_from(s_0_21).unwrap());
        // D s_0_23: cmp-eq s_0_22 s_0_20
        let s_0_23: bool = ((s_0_22) == (s_0_20));
        // N s_0_24: branch s_0_23 b43 b1
        if s_0_23 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #64s : i64
        let s_1_0: i64 = 64;
        // D s_1_1: read-var n:i64
        let s_1_1: i64 = fn_state.n;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: call X_read(s_1_2, s_1_0)
        let s_1_3: Bits = X_read(state, tracer, s_1_2, s_1_0);
        // D s_1_4: cast reint s_1_3 -> u64
        let s_1_4: u64 = (s_1_3.value() as u64);
        // D s_1_5: write-var address <= s_1_4
        fn_state.address = s_1_4;
        // N s_1_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var postindex:u8
        let s_2_0: bool = fn_state.postindex;
        // D s_2_1: not s_2_0
        let s_2_1: bool = !s_2_0;
        // N s_2_2: branch s_2_1 b42 b3
        if s_2_1 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #1u : u32
        let s_4_0: u32 = 1;
        // D s_4_1: read-var memop:u32
        let s_4_1: u32 = fn_state.memop;
        // D s_4_2: cmp-eq s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) == (s_4_1));
        // D s_4_3: not s_4_2
        let s_4_3: bool = !s_4_2;
        // N s_4_4: branch s_4_3 b25 b5
        if s_4_3 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var rt_unknown:u8
        let s_5_0: bool = fn_state.rt_unknown;
        // N s_5_1: branch s_5_0 b24 b6
        if s_5_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var datasizeshadow#1781:i64
        let s_6_0: i64 = fn_state.datasizeshadow_1781;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // D s_6_3: read-var t:i64
        let s_6_3: i64 = fn_state.t;
        // D s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_5: call X_read(s_6_4, s_6_2)
        let s_6_5: Bits = X_read(state, tracer, s_6_4, s_6_2);
        // D s_6_6: write-var data <= s_6_5
        fn_state.data = s_6_5;
        // N s_6_7: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var wback:u8
        let s_7_0: bool = fn_state.wback;
        // D s_7_1: not s_7_0
        let s_7_1: bool = !s_7_0;
        // N s_7_2: branch s_7_1 b23 b8
        if s_7_1 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #8s : i
        let s_9_0: i128 = 8;
        // D s_9_1: read-var datasizeshadow#1781:i64
        let s_9_1: i64 = fn_state.datasizeshadow_1781;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: div s_9_2 s_9_0
        let s_9_3: i128 = ((s_9_2) / (s_9_0));
        // D s_9_4: cast reint s_9_3 -> i64
        let s_9_4: i64 = (s_9_3 as i64);
        // D s_9_5: cast zx s_9_4 -> i
        let s_9_5: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_6: read-var address:u64
        let s_9_6: u64 = fn_state.address;
        // D s_9_7: read-var accdesc:struct
        let s_9_7: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_9_8: read-var data:bv
        let s_9_8: Bits = fn_state.data;
        // D s_9_9: call Mem_set(s_9_6, s_9_5, s_9_7, s_9_8)
        let s_9_9: () = Mem_set(state, tracer, s_9_6, s_9_5, s_9_7, s_9_8);
        // N s_9_10: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var wback:u8
        let s_10_0: bool = fn_state.wback;
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
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #22416u : u32
        let s_12_0: u32 = 22416;
        // D s_12_1: read-reg s_12_0:u8
        let s_12_1: bool = {
            let value = state.read_register::<bool>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // N s_12_2: branch s_12_1 b14 b13
        if s_12_1 {
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
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call SPESampleGeneralPurposeLoadStore(s_14_0)
        let s_14_1: () = SPESampleGeneralPurposeLoadStore(state, tracer, s_14_0);
        // N s_14_2: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var wb_unknown:u8
        let s_15_0: bool = fn_state.wb_unknown;
        // N s_15_1: branch s_15_0 b22 b16
        if s_15_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var postindex:u8
        let s_16_0: bool = fn_state.postindex;
        // N s_16_1: branch s_16_0 b21 b17
        if s_16_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #31s : i
        let s_18_0: i128 = 31;
        // D s_18_1: read-var n:i64
        let s_18_1: i64 = fn_state.n;
        // D s_18_2: cast zx s_18_1 -> i
        let s_18_2: i128 = (i128::try_from(s_18_1).unwrap());
        // D s_18_3: cmp-eq s_18_2 s_18_0
        let s_18_3: bool = ((s_18_2) == (s_18_0));
        // N s_18_4: branch s_18_3 b20 b19
        if s_18_3 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #64s : i64
        let s_19_0: i64 = 64;
        // D s_19_1: read-var n:i64
        let s_19_1: i64 = fn_state.n;
        // D s_19_2: cast zx s_19_1 -> i
        let s_19_2: i128 = (i128::try_from(s_19_1).unwrap());
        // D s_19_3: read-var address:u64
        let s_19_3: u64 = fn_state.address;
        // D s_19_4: cast zx s_19_3 -> bv
        let s_19_4: Bits = Bits::new(s_19_3 as u128, 64u16);
        // D s_19_5: call X_set(s_19_2, s_19_0, s_19_4)
        let s_19_5: () = X_set(state, tracer, s_19_2, s_19_0, s_19_4);
        // N s_19_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var address:u64
        let s_20_0: u64 = fn_state.address;
        // D s_20_1: call SP_set(s_20_0)
        let s_20_1: () = SP_set(state, tracer, s_20_0);
        // N s_20_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var address:u64
        let s_21_0: u64 = fn_state.address;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 64u16);
        // D s_21_2: read-var offset:u64
        let s_21_2: u64 = fn_state.offset;
        // D s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 64u16);
        // D s_21_4: add s_21_1 s_21_3
        let s_21_4: Bits = (s_21_1 + s_21_3);
        // D s_21_5: cast reint s_21_4 -> u64
        let s_21_5: u64 = (s_21_4.value() as u64);
        // D s_21_6: write-var address <= s_21_5
        fn_state.address = s_21_5;
        // N s_21_7: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #64s : i64
        let s_22_0: i64 = 64;
        // C s_22_1: cast zx s_22_0 -> i
        let s_22_1: i128 = (i128::try_from(s_22_0).unwrap());
        // S s_22_2: call __UNKNOWN_bits(s_22_1)
        let s_22_2: Bits = u__UNKNOWN_bits(state, tracer, s_22_1);
        // S s_22_3: cast reint s_22_2 -> u64
        let s_22_3: u64 = (s_22_2.value() as u64);
        // D s_22_4: write-var address <= s_22_3
        fn_state.address = s_22_3;
        // N s_22_5: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #8s : i
        let s_23_0: i128 = 8;
        // D s_23_1: read-var datasizeshadow#1781:i64
        let s_23_1: i64 = fn_state.datasizeshadow_1781;
        // D s_23_2: cast zx s_23_1 -> i
        let s_23_2: i128 = (i128::try_from(s_23_1).unwrap());
        // D s_23_3: div s_23_2 s_23_0
        let s_23_3: i128 = ((s_23_2) / (s_23_0));
        // D s_23_4: cast reint s_23_3 -> i64
        let s_23_4: i64 = (s_23_3 as i64);
        // C s_23_5: const #64s : i
        let s_23_5: i128 = 64;
        // D s_23_6: read-var regsizeshadow#1780:i64
        let s_23_6: i64 = fn_state.regsizeshadow_1780;
        // D s_23_7: cast zx s_23_6 -> i
        let s_23_7: i128 = (i128::try_from(s_23_6).unwrap());
        // D s_23_8: cmp-eq s_23_7 s_23_5
        let s_23_8: bool = ((s_23_7) == (s_23_5));
        // D s_23_9: cast zx s_23_4 -> i
        let s_23_9: i128 = (i128::try_from(s_23_4).unwrap());
        // D s_23_10: read-var t:i64
        let s_23_10: i64 = fn_state.t;
        // D s_23_11: cast zx s_23_10 -> i
        let s_23_11: i128 = (i128::try_from(s_23_10).unwrap());
        // C s_23_12: const #0u : u8
        let s_23_12: bool = false;
        // C s_23_13: const #0u : u8
        let s_23_13: bool = false;
        // D s_23_14: call AArch64_SetLSInstructionSyndrome(s_23_9, s_23_12, s_23_11, s_23_8, s_23_13)
        let s_23_14: () = AArch64_SetLSInstructionSyndrome(
            state,
            tracer,
            s_23_9,
            s_23_12,
            s_23_11,
            s_23_8,
            s_23_13,
        );
        // N s_23_15: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var datasizeshadow#1781:i64
        let s_24_0: i64 = fn_state.datasizeshadow_1781;
        // D s_24_1: cast zx s_24_0 -> i
        let s_24_1: i128 = (i128::try_from(s_24_0).unwrap());
        // D s_24_2: cast reint s_24_1 -> i64
        let s_24_2: i64 = (s_24_1 as i64);
        // D s_24_3: cast zx s_24_2 -> i
        let s_24_3: i128 = (i128::try_from(s_24_2).unwrap());
        // D s_24_4: call __UNKNOWN_bits(s_24_3)
        let s_24_4: Bits = u__UNKNOWN_bits(state, tracer, s_24_3);
        // D s_24_5: write-var data <= s_24_4
        fn_state.data = s_24_4;
        // N s_24_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0u : u32
        let s_25_0: u32 = 0;
        // D s_25_1: read-var memop:u32
        let s_25_1: u32 = fn_state.memop;
        // D s_25_2: cmp-eq s_25_0 s_25_1
        let s_25_2: bool = ((s_25_0) == (s_25_1));
        // D s_25_3: not s_25_2
        let s_25_3: bool = !s_25_2;
        // N s_25_4: branch s_25_3 b39 b26
        if s_25_3 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var wback:u8
        let s_26_0: bool = fn_state.wback;
        // D s_26_1: not s_26_0
        let s_26_1: bool = !s_26_0;
        // N s_26_2: branch s_26_1 b38 b27
        if s_26_1 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #8s : i
        let s_28_0: i128 = 8;
        // D s_28_1: read-var datasizeshadow#1781:i64
        let s_28_1: i64 = fn_state.datasizeshadow_1781;
        // D s_28_2: cast zx s_28_1 -> i
        let s_28_2: i128 = (i128::try_from(s_28_1).unwrap());
        // D s_28_3: div s_28_2 s_28_0
        let s_28_3: i128 = ((s_28_2) / (s_28_0));
        // D s_28_4: cast reint s_28_3 -> i64
        let s_28_4: i64 = (s_28_3 as i64);
        // D s_28_5: cast zx s_28_4 -> i
        let s_28_5: i128 = (i128::try_from(s_28_4).unwrap());
        // D s_28_6: read-var address:u64
        let s_28_6: u64 = fn_state.address;
        // D s_28_7: read-var accdesc:struct
        let s_28_7: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_28_8: call Mem_read(s_28_6, s_28_5, s_28_7)
        let s_28_8: Bits = Mem_read(state, tracer, s_28_6, s_28_5, s_28_7);
        // D s_28_9: write-var datashadow#1782 <= s_28_8
        fn_state.datashadow_1782 = s_28_8;
        // N s_28_10: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var is_signed:u8
        let s_29_0: bool = fn_state.is_signed;
        // N s_29_1: branch s_29_0 b34 b30
        if s_29_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var datasizeshadow#1781:i64
        let s_30_0: i64 = fn_state.datasizeshadow_1781;
        // D s_30_1: cast zx s_30_0 -> i
        let s_30_1: i128 = (i128::try_from(s_30_0).unwrap());
        // D s_30_2: call __id(s_30_1)
        let s_30_2: i128 = u__id(state, tracer, s_30_1);
        // D s_30_3: cast reint s_30_2 -> i64
        let s_30_3: i64 = (s_30_2 as i64);
        // C s_30_4: const #0s : i
        let s_30_4: i128 = 0;
        // D s_30_5: cast zx s_30_3 -> i
        let s_30_5: i128 = (i128::try_from(s_30_3).unwrap());
        // D s_30_6: cmp-ge s_30_5 s_30_4
        let s_30_6: bool = ((s_30_5) >= (s_30_4));
        // N s_30_7: branch s_30_6 b33 b31
        if s_30_6 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #0u : u8
        let s_31_0: bool = false;
        // D s_31_1: write-var gs#165739 <= s_31_0
        fn_state.gs_165739 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#165739:u8
        let s_32_0: bool = fn_state.gs_165739;
        // N s_32_1: assert s_32_0
        let s_32_1: () = assert!(s_32_0);
        // D s_32_2: read-var regsizeshadow#1780:i64
        let s_32_2: i64 = fn_state.regsizeshadow_1780;
        // D s_32_3: cast zx s_32_2 -> i
        let s_32_3: i128 = (i128::try_from(s_32_2).unwrap());
        // D s_32_4: cast reint s_32_3 -> i64
        let s_32_4: i64 = (s_32_3 as i64);
        // D s_32_5: read-var regsizeshadow#1780:i64
        let s_32_5: i64 = fn_state.regsizeshadow_1780;
        // D s_32_6: cast zx s_32_5 -> i
        let s_32_6: i128 = (i128::try_from(s_32_5).unwrap());
        // D s_32_7: read-var datashadow#1782:bv
        let s_32_7: Bits = fn_state.datashadow_1782;
        // D s_32_8: bits-cast zx s_32_7 -> bv length s_32_6
        let s_32_8: Bits = s_32_7.zero_extend(s_32_6);
        // D s_32_9: read-var t:i64
        let s_32_9: i64 = fn_state.t;
        // D s_32_10: cast zx s_32_9 -> i
        let s_32_10: i128 = (i128::try_from(s_32_9).unwrap());
        // D s_32_11: call X_set(s_32_10, s_32_4, s_32_8)
        let s_32_11: () = X_set(state, tracer, s_32_10, s_32_4, s_32_8);
        // N s_32_12: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var regsizeshadow#1780:i64
        let s_33_0: i64 = fn_state.regsizeshadow_1780;
        // D s_33_1: cast zx s_33_0 -> i
        let s_33_1: i128 = (i128::try_from(s_33_0).unwrap());
        // D s_33_2: call __id(s_33_1)
        let s_33_2: i128 = u__id(state, tracer, s_33_1);
        // D s_33_3: cast reint s_33_2 -> i64
        let s_33_3: i64 = (s_33_2 as i64);
        // D s_33_4: read-var datasizeshadow#1781:i64
        let s_33_4: i64 = fn_state.datasizeshadow_1781;
        // D s_33_5: cast zx s_33_4 -> i
        let s_33_5: i128 = (i128::try_from(s_33_4).unwrap());
        // D s_33_6: call __id(s_33_5)
        let s_33_6: i128 = u__id(state, tracer, s_33_5);
        // D s_33_7: cast reint s_33_6 -> i64
        let s_33_7: i64 = (s_33_6 as i64);
        // D s_33_8: cast zx s_33_3 -> i
        let s_33_8: i128 = (i128::try_from(s_33_3).unwrap());
        // D s_33_9: cast zx s_33_7 -> i
        let s_33_9: i128 = (i128::try_from(s_33_7).unwrap());
        // D s_33_10: cmp-ge s_33_8 s_33_9
        let s_33_10: bool = ((s_33_8) >= (s_33_9));
        // D s_33_11: write-var gs#165739 <= s_33_10
        fn_state.gs_165739 = s_33_10;
        // N s_33_12: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var datasizeshadow#1781:i64
        let s_34_0: i64 = fn_state.datasizeshadow_1781;
        // D s_34_1: cast zx s_34_0 -> i
        let s_34_1: i128 = (i128::try_from(s_34_0).unwrap());
        // D s_34_2: call __id(s_34_1)
        let s_34_2: i128 = u__id(state, tracer, s_34_1);
        // D s_34_3: cast reint s_34_2 -> i64
        let s_34_3: i64 = (s_34_2 as i64);
        // C s_34_4: const #0s : i
        let s_34_4: i128 = 0;
        // D s_34_5: cast zx s_34_3 -> i
        let s_34_5: i128 = (i128::try_from(s_34_3).unwrap());
        // D s_34_6: cmp-gt s_34_5 s_34_4
        let s_34_6: bool = ((s_34_5) > (s_34_4));
        // N s_34_7: branch s_34_6 b37 b35
        if s_34_6 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // D s_35_1: write-var gs#165742 <= s_35_0
        fn_state.gs_165742 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#165742:u8
        let s_36_0: bool = fn_state.gs_165742;
        // N s_36_1: assert s_36_0
        let s_36_1: () = assert!(s_36_0);
        // D s_36_2: read-var regsizeshadow#1780:i64
        let s_36_2: i64 = fn_state.regsizeshadow_1780;
        // D s_36_3: cast zx s_36_2 -> i
        let s_36_3: i128 = (i128::try_from(s_36_2).unwrap());
        // D s_36_4: cast reint s_36_3 -> i64
        let s_36_4: i64 = (s_36_3 as i64);
        // D s_36_5: read-var regsizeshadow#1780:i64
        let s_36_5: i64 = fn_state.regsizeshadow_1780;
        // D s_36_6: cast zx s_36_5 -> i
        let s_36_6: i128 = (i128::try_from(s_36_5).unwrap());
        // D s_36_7: read-var datashadow#1782:bv
        let s_36_7: Bits = fn_state.datashadow_1782;
        // D s_36_8: bits-cast sx s_36_7 -> bv length s_36_6
        let s_36_8: Bits = s_36_7.sign_extend(s_36_6);
        // D s_36_9: read-var t:i64
        let s_36_9: i64 = fn_state.t;
        // D s_36_10: cast zx s_36_9 -> i
        let s_36_10: i128 = (i128::try_from(s_36_9).unwrap());
        // D s_36_11: call X_set(s_36_10, s_36_4, s_36_8)
        let s_36_11: () = X_set(state, tracer, s_36_10, s_36_4, s_36_8);
        // N s_36_12: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var regsizeshadow#1780:i64
        let s_37_0: i64 = fn_state.regsizeshadow_1780;
        // D s_37_1: cast zx s_37_0 -> i
        let s_37_1: i128 = (i128::try_from(s_37_0).unwrap());
        // D s_37_2: call __id(s_37_1)
        let s_37_2: i128 = u__id(state, tracer, s_37_1);
        // D s_37_3: cast reint s_37_2 -> i64
        let s_37_3: i64 = (s_37_2 as i64);
        // D s_37_4: read-var datasizeshadow#1781:i64
        let s_37_4: i64 = fn_state.datasizeshadow_1781;
        // D s_37_5: cast zx s_37_4 -> i
        let s_37_5: i128 = (i128::try_from(s_37_4).unwrap());
        // D s_37_6: call __id(s_37_5)
        let s_37_6: i128 = u__id(state, tracer, s_37_5);
        // D s_37_7: cast reint s_37_6 -> i64
        let s_37_7: i64 = (s_37_6 as i64);
        // D s_37_8: cast zx s_37_3 -> i
        let s_37_8: i128 = (i128::try_from(s_37_3).unwrap());
        // D s_37_9: cast zx s_37_7 -> i
        let s_37_9: i128 = (i128::try_from(s_37_7).unwrap());
        // D s_37_10: cmp-ge s_37_8 s_37_9
        let s_37_10: bool = ((s_37_8) >= (s_37_9));
        // D s_37_11: write-var gs#165742 <= s_37_10
        fn_state.gs_165742 = s_37_10;
        // N s_37_12: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #8s : i
        let s_38_0: i128 = 8;
        // D s_38_1: read-var datasizeshadow#1781:i64
        let s_38_1: i64 = fn_state.datasizeshadow_1781;
        // D s_38_2: cast zx s_38_1 -> i
        let s_38_2: i128 = (i128::try_from(s_38_1).unwrap());
        // D s_38_3: div s_38_2 s_38_0
        let s_38_3: i128 = ((s_38_2) / (s_38_0));
        // D s_38_4: cast reint s_38_3 -> i64
        let s_38_4: i64 = (s_38_3 as i64);
        // C s_38_5: const #64s : i
        let s_38_5: i128 = 64;
        // D s_38_6: read-var regsizeshadow#1780:i64
        let s_38_6: i64 = fn_state.regsizeshadow_1780;
        // D s_38_7: cast zx s_38_6 -> i
        let s_38_7: i128 = (i128::try_from(s_38_6).unwrap());
        // D s_38_8: cmp-eq s_38_7 s_38_5
        let s_38_8: bool = ((s_38_7) == (s_38_5));
        // D s_38_9: cast zx s_38_4 -> i
        let s_38_9: i128 = (i128::try_from(s_38_4).unwrap());
        // D s_38_10: read-var t:i64
        let s_38_10: i64 = fn_state.t;
        // D s_38_11: cast zx s_38_10 -> i
        let s_38_11: i128 = (i128::try_from(s_38_10).unwrap());
        // D s_38_12: read-var is_signed:u8
        let s_38_12: bool = fn_state.is_signed;
        // C s_38_13: const #0u : u8
        let s_38_13: bool = false;
        // D s_38_14: call AArch64_SetLSInstructionSyndrome(s_38_9, s_38_12, s_38_11, s_38_8, s_38_13)
        let s_38_14: () = AArch64_SetLSInstructionSyndrome(
            state,
            tracer,
            s_38_9,
            s_38_12,
            s_38_11,
            s_38_8,
            s_38_13,
        );
        // N s_38_15: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #2u : u32
        let s_39_0: u32 = 2;
        // D s_39_1: read-var memop:u32
        let s_39_1: u32 = fn_state.memop;
        // D s_39_2: cmp-eq s_39_0 s_39_1
        let s_39_2: bool = ((s_39_0) == (s_39_1));
        // D s_39_3: not s_39_2
        let s_39_3: bool = !s_39_2;
        // N s_39_4: branch s_39_3 b41 b40
        if s_39_3 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #4s : i
        let s_40_0: i128 = 4;
        // C s_40_1: const #0s : i
        let s_40_1: i128 = 0;
        // D s_40_2: read-var t:i64
        let s_40_2: i64 = fn_state.t;
        // D s_40_3: cast zx s_40_2 -> i
        let s_40_3: i128 = (i128::try_from(s_40_2).unwrap());
        // D s_40_4: call integer_subrange(s_40_3, s_40_0, s_40_1)
        let s_40_4: Bits = integer_subrange(state, tracer, s_40_3, s_40_0, s_40_1);
        // D s_40_5: cast reint s_40_4 -> u8
        let s_40_5: u8 = (s_40_4.value() as u8);
        // D s_40_6: read-var address:u64
        let s_40_6: u64 = fn_state.address;
        // D s_40_7: call Prefetch(s_40_6, s_40_5)
        let s_40_7: () = Prefetch(state, tracer, s_40_6, s_40_5);
        // N s_40_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_41_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var address:u64
        let s_42_0: u64 = fn_state.address;
        // D s_42_1: cast zx s_42_0 -> bv
        let s_42_1: Bits = Bits::new(s_42_0 as u128, 64u16);
        // D s_42_2: read-var offset:u64
        let s_42_2: u64 = fn_state.offset;
        // D s_42_3: cast zx s_42_2 -> bv
        let s_42_3: Bits = Bits::new(s_42_2 as u128, 64u16);
        // D s_42_4: add s_42_1 s_42_3
        let s_42_4: Bits = (s_42_1 + s_42_3);
        // D s_42_5: cast reint s_42_4 -> u64
        let s_42_5: u64 = (s_42_4.value() as u64);
        // D s_42_6: write-var address <= s_42_5
        fn_state.address = s_42_5;
        // N s_42_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var memop:u32
        let s_43_0: u32 = fn_state.memop;
        // C s_43_1: const #2u : u32
        let s_43_1: u32 = 2;
        // D s_43_2: cmp-eq s_43_0 s_43_1
        let s_43_2: bool = ((s_43_0) == (s_43_1));
        // N s_43_3: branch s_43_2 b46 b44
        if s_43_2 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_44_0: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #() : ()
        let s_45_0: () = ();
        // S s_45_1: call SP_read(s_45_0)
        let s_45_1: u64 = SP_read(state, tracer, s_45_0);
        // D s_45_2: write-var address <= s_45_1
        fn_state.address = s_45_1;
        // N s_45_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #() : ()
        let s_46_0: () = ();
        // S s_46_1: call CheckSPAlignment(s_46_0)
        let s_46_1: () = CheckSPAlignment(state, tracer, s_46_0);
        // N s_46_2: jump b45
        return block_45(state, tracer, fn_state);
    }
}
