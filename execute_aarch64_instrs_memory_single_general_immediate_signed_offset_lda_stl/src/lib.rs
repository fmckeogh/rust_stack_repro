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
use SP_set::*;
use CreateAccDescAcqRel::*;
use X_read::*;
use AArch64_SetLSInstructionSyndrome::*;
use Mem_set::*;
use X_set::*;
use u__UNKNOWN_bits::*;
use u__id::*;
use Mem_read::*;
use SP_read::*;
use SPESampleGeneralPurposeLoadStore::*;
use integer_subrange::*;
use CreateAccDescLDAcqPC::*;
use Prefetch::*;
use CheckSPAlignment::*;
use common::*;
pub fn execute_aarch64_instrs_memory_single_general_immediate_signed_offset_lda_stl<
    T: Tracer,
>(
    state: &mut State,
    tracer: &T,
    datasize: i64,
    memop: u32,
    n: i64,
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
        datasizeshadow_1609: i64,
        address: u64,
        datashadow_1610: Bits,
        gs_158642: bool,
        regsizeshadow_1608: i64,
        data: Bits,
        accdesc: ProductType9878976b5bcce9c9,
        gs_158639: bool,
        datasize: i64,
        memop: u32,
        n: i64,
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
        // D s_0_3: write-var regsizeshadow#1608 <= s_0_2
        fn_state.regsizeshadow_1608 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1609 <= s_0_6
        fn_state.datasizeshadow_1609 = s_0_6;
        // D s_0_8: read-var memop:u32
        let s_0_8: u32 = fn_state.memop;
        // C s_0_9: const #0u : u32
        let s_0_9: u32 = 0;
        // D s_0_10: cmp-eq s_0_8 s_0_9
        let s_0_10: bool = ((s_0_8) == (s_0_9));
        // N s_0_11: branch s_0_10 b51 b1
        if s_0_10 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var memop:u32
        let s_1_0: u32 = fn_state.memop;
        // C s_1_1: const #1u : u32
        let s_1_1: u32 = 1;
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // N s_1_3: branch s_1_2 b50 b2
        if s_1_2 {
            return block_50(state, tracer, fn_state);
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
        // C s_3_0: const #31s : i
        let s_3_0: i128 = 31;
        // D s_3_1: read-var n:i64
        let s_3_1: i64 = fn_state.n;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: cmp-eq s_3_2 s_3_0
        let s_3_3: bool = ((s_3_2) == (s_3_0));
        // N s_3_4: branch s_3_3 b46 b4
        if s_3_3 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #64s : i64
        let s_4_0: i64 = 64;
        // D s_4_1: read-var n:i64
        let s_4_1: i64 = fn_state.n;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: call X_read(s_4_2, s_4_0)
        let s_4_3: Bits = X_read(state, tracer, s_4_2, s_4_0);
        // D s_4_4: cast reint s_4_3 -> u64
        let s_4_4: u64 = (s_4_3.value() as u64);
        // D s_4_5: write-var address <= s_4_4
        fn_state.address = s_4_4;
        // N s_4_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var postindex:u8
        let s_5_0: bool = fn_state.postindex;
        // D s_5_1: not s_5_0
        let s_5_1: bool = !s_5_0;
        // N s_5_2: branch s_5_1 b45 b6
        if s_5_1 {
            return block_45(state, tracer, fn_state);
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
        // C s_7_0: const #1u : u32
        let s_7_0: u32 = 1;
        // D s_7_1: read-var memop:u32
        let s_7_1: u32 = fn_state.memop;
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // D s_7_3: not s_7_2
        let s_7_3: bool = !s_7_2;
        // N s_7_4: branch s_7_3 b28 b8
        if s_7_3 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var rt_unknown:u8
        let s_8_0: bool = fn_state.rt_unknown;
        // N s_8_1: branch s_8_0 b27 b9
        if s_8_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var datasizeshadow#1609:i64
        let s_9_0: i64 = fn_state.datasizeshadow_1609;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: cast reint s_9_1 -> i64
        let s_9_2: i64 = (s_9_1 as i64);
        // D s_9_3: read-var t:i64
        let s_9_3: i64 = fn_state.t;
        // D s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_5: call X_read(s_9_4, s_9_2)
        let s_9_5: Bits = X_read(state, tracer, s_9_4, s_9_2);
        // D s_9_6: write-var data <= s_9_5
        fn_state.data = s_9_5;
        // N s_9_7: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var wback:u8
        let s_10_0: bool = fn_state.wback;
        // D s_10_1: not s_10_0
        let s_10_1: bool = !s_10_0;
        // N s_10_2: branch s_10_1 b26 b11
        if s_10_1 {
            return block_26(state, tracer, fn_state);
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
        // C s_12_0: const #8s : i
        let s_12_0: i128 = 8;
        // D s_12_1: read-var datasizeshadow#1609:i64
        let s_12_1: i64 = fn_state.datasizeshadow_1609;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: div s_12_2 s_12_0
        let s_12_3: i128 = ((s_12_2) / (s_12_0));
        // D s_12_4: cast reint s_12_3 -> i64
        let s_12_4: i64 = (s_12_3 as i64);
        // D s_12_5: cast zx s_12_4 -> i
        let s_12_5: i128 = (i128::try_from(s_12_4).unwrap());
        // D s_12_6: read-var address:u64
        let s_12_6: u64 = fn_state.address;
        // D s_12_7: read-var accdesc:struct
        let s_12_7: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_12_8: read-var data:bv
        let s_12_8: Bits = fn_state.data;
        // D s_12_9: call Mem_set(s_12_6, s_12_5, s_12_7, s_12_8)
        let s_12_9: () = Mem_set(state, tracer, s_12_6, s_12_5, s_12_7, s_12_8);
        // N s_12_10: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var wback:u8
        let s_13_0: bool = fn_state.wback;
        // N s_13_1: branch s_13_0 b18 b14
        if s_13_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #22416u : u32
        let s_15_0: u32 = 22416;
        // D s_15_1: read-reg s_15_0:u8
        let s_15_1: bool = {
            let value = state.read_register::<bool>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // N s_15_2: branch s_15_1 b17 b16
        if s_15_1 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call SPESampleGeneralPurposeLoadStore(s_17_0)
        let s_17_1: () = SPESampleGeneralPurposeLoadStore(state, tracer, s_17_0);
        // N s_17_2: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var wb_unknown:u8
        let s_18_0: bool = fn_state.wb_unknown;
        // N s_18_1: branch s_18_0 b25 b19
        if s_18_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var postindex:u8
        let s_19_0: bool = fn_state.postindex;
        // N s_19_1: branch s_19_0 b24 b20
        if s_19_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #31s : i
        let s_21_0: i128 = 31;
        // D s_21_1: read-var n:i64
        let s_21_1: i64 = fn_state.n;
        // D s_21_2: cast zx s_21_1 -> i
        let s_21_2: i128 = (i128::try_from(s_21_1).unwrap());
        // D s_21_3: cmp-eq s_21_2 s_21_0
        let s_21_3: bool = ((s_21_2) == (s_21_0));
        // N s_21_4: branch s_21_3 b23 b22
        if s_21_3 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #64s : i64
        let s_22_0: i64 = 64;
        // D s_22_1: read-var n:i64
        let s_22_1: i64 = fn_state.n;
        // D s_22_2: cast zx s_22_1 -> i
        let s_22_2: i128 = (i128::try_from(s_22_1).unwrap());
        // D s_22_3: read-var address:u64
        let s_22_3: u64 = fn_state.address;
        // D s_22_4: cast zx s_22_3 -> bv
        let s_22_4: Bits = Bits::new(s_22_3 as u128, 64u16);
        // D s_22_5: call X_set(s_22_2, s_22_0, s_22_4)
        let s_22_5: () = X_set(state, tracer, s_22_2, s_22_0, s_22_4);
        // N s_22_6: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var address:u64
        let s_23_0: u64 = fn_state.address;
        // D s_23_1: call SP_set(s_23_0)
        let s_23_1: () = SP_set(state, tracer, s_23_0);
        // N s_23_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var address:u64
        let s_24_0: u64 = fn_state.address;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 64u16);
        // D s_24_2: read-var offset:u64
        let s_24_2: u64 = fn_state.offset;
        // D s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 64u16);
        // D s_24_4: add s_24_1 s_24_3
        let s_24_4: Bits = (s_24_1 + s_24_3);
        // D s_24_5: cast reint s_24_4 -> u64
        let s_24_5: u64 = (s_24_4.value() as u64);
        // D s_24_6: write-var address <= s_24_5
        fn_state.address = s_24_5;
        // N s_24_7: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #64s : i64
        let s_25_0: i64 = 64;
        // C s_25_1: cast zx s_25_0 -> i
        let s_25_1: i128 = (i128::try_from(s_25_0).unwrap());
        // S s_25_2: call __UNKNOWN_bits(s_25_1)
        let s_25_2: Bits = u__UNKNOWN_bits(state, tracer, s_25_1);
        // S s_25_3: cast reint s_25_2 -> u64
        let s_25_3: u64 = (s_25_2.value() as u64);
        // D s_25_4: write-var address <= s_25_3
        fn_state.address = s_25_3;
        // N s_25_5: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #8s : i
        let s_26_0: i128 = 8;
        // D s_26_1: read-var datasizeshadow#1609:i64
        let s_26_1: i64 = fn_state.datasizeshadow_1609;
        // D s_26_2: cast zx s_26_1 -> i
        let s_26_2: i128 = (i128::try_from(s_26_1).unwrap());
        // D s_26_3: div s_26_2 s_26_0
        let s_26_3: i128 = ((s_26_2) / (s_26_0));
        // D s_26_4: cast reint s_26_3 -> i64
        let s_26_4: i64 = (s_26_3 as i64);
        // C s_26_5: const #64s : i
        let s_26_5: i128 = 64;
        // D s_26_6: read-var regsizeshadow#1608:i64
        let s_26_6: i64 = fn_state.regsizeshadow_1608;
        // D s_26_7: cast zx s_26_6 -> i
        let s_26_7: i128 = (i128::try_from(s_26_6).unwrap());
        // D s_26_8: cmp-eq s_26_7 s_26_5
        let s_26_8: bool = ((s_26_7) == (s_26_5));
        // D s_26_9: cast zx s_26_4 -> i
        let s_26_9: i128 = (i128::try_from(s_26_4).unwrap());
        // D s_26_10: read-var t:i64
        let s_26_10: i64 = fn_state.t;
        // D s_26_11: cast zx s_26_10 -> i
        let s_26_11: i128 = (i128::try_from(s_26_10).unwrap());
        // C s_26_12: const #0u : u8
        let s_26_12: bool = false;
        // C s_26_13: const #0u : u8
        let s_26_13: bool = false;
        // D s_26_14: call AArch64_SetLSInstructionSyndrome(s_26_9, s_26_12, s_26_11, s_26_8, s_26_13)
        let s_26_14: () = AArch64_SetLSInstructionSyndrome(
            state,
            tracer,
            s_26_9,
            s_26_12,
            s_26_11,
            s_26_8,
            s_26_13,
        );
        // N s_26_15: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var datasizeshadow#1609:i64
        let s_27_0: i64 = fn_state.datasizeshadow_1609;
        // D s_27_1: cast zx s_27_0 -> i
        let s_27_1: i128 = (i128::try_from(s_27_0).unwrap());
        // D s_27_2: cast reint s_27_1 -> i64
        let s_27_2: i64 = (s_27_1 as i64);
        // D s_27_3: cast zx s_27_2 -> i
        let s_27_3: i128 = (i128::try_from(s_27_2).unwrap());
        // D s_27_4: call __UNKNOWN_bits(s_27_3)
        let s_27_4: Bits = u__UNKNOWN_bits(state, tracer, s_27_3);
        // D s_27_5: write-var data <= s_27_4
        fn_state.data = s_27_4;
        // N s_27_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #0u : u32
        let s_28_0: u32 = 0;
        // D s_28_1: read-var memop:u32
        let s_28_1: u32 = fn_state.memop;
        // D s_28_2: cmp-eq s_28_0 s_28_1
        let s_28_2: bool = ((s_28_0) == (s_28_1));
        // D s_28_3: not s_28_2
        let s_28_3: bool = !s_28_2;
        // N s_28_4: branch s_28_3 b42 b29
        if s_28_3 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var wback:u8
        let s_29_0: bool = fn_state.wback;
        // D s_29_1: not s_29_0
        let s_29_1: bool = !s_29_0;
        // N s_29_2: branch s_29_1 b41 b30
        if s_29_1 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_30_0: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #8s : i
        let s_31_0: i128 = 8;
        // D s_31_1: read-var datasizeshadow#1609:i64
        let s_31_1: i64 = fn_state.datasizeshadow_1609;
        // D s_31_2: cast zx s_31_1 -> i
        let s_31_2: i128 = (i128::try_from(s_31_1).unwrap());
        // D s_31_3: div s_31_2 s_31_0
        let s_31_3: i128 = ((s_31_2) / (s_31_0));
        // D s_31_4: cast reint s_31_3 -> i64
        let s_31_4: i64 = (s_31_3 as i64);
        // D s_31_5: cast zx s_31_4 -> i
        let s_31_5: i128 = (i128::try_from(s_31_4).unwrap());
        // D s_31_6: read-var address:u64
        let s_31_6: u64 = fn_state.address;
        // D s_31_7: read-var accdesc:struct
        let s_31_7: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_31_8: call Mem_read(s_31_6, s_31_5, s_31_7)
        let s_31_8: Bits = Mem_read(state, tracer, s_31_6, s_31_5, s_31_7);
        // D s_31_9: write-var datashadow#1610 <= s_31_8
        fn_state.datashadow_1610 = s_31_8;
        // N s_31_10: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var is_signed:u8
        let s_32_0: bool = fn_state.is_signed;
        // N s_32_1: branch s_32_0 b37 b33
        if s_32_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var datasizeshadow#1609:i64
        let s_33_0: i64 = fn_state.datasizeshadow_1609;
        // D s_33_1: cast zx s_33_0 -> i
        let s_33_1: i128 = (i128::try_from(s_33_0).unwrap());
        // D s_33_2: call __id(s_33_1)
        let s_33_2: i128 = u__id(state, tracer, s_33_1);
        // D s_33_3: cast reint s_33_2 -> i64
        let s_33_3: i64 = (s_33_2 as i64);
        // C s_33_4: const #0s : i
        let s_33_4: i128 = 0;
        // D s_33_5: cast zx s_33_3 -> i
        let s_33_5: i128 = (i128::try_from(s_33_3).unwrap());
        // D s_33_6: cmp-ge s_33_5 s_33_4
        let s_33_6: bool = ((s_33_5) >= (s_33_4));
        // N s_33_7: branch s_33_6 b36 b34
        if s_33_6 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var gs#158639 <= s_34_0
        fn_state.gs_158639 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#158639:u8
        let s_35_0: bool = fn_state.gs_158639;
        // N s_35_1: assert s_35_0
        let s_35_1: () = assert!(s_35_0);
        // D s_35_2: read-var regsizeshadow#1608:i64
        let s_35_2: i64 = fn_state.regsizeshadow_1608;
        // D s_35_3: cast zx s_35_2 -> i
        let s_35_3: i128 = (i128::try_from(s_35_2).unwrap());
        // D s_35_4: cast reint s_35_3 -> i64
        let s_35_4: i64 = (s_35_3 as i64);
        // D s_35_5: read-var regsizeshadow#1608:i64
        let s_35_5: i64 = fn_state.regsizeshadow_1608;
        // D s_35_6: cast zx s_35_5 -> i
        let s_35_6: i128 = (i128::try_from(s_35_5).unwrap());
        // D s_35_7: read-var datashadow#1610:bv
        let s_35_7: Bits = fn_state.datashadow_1610;
        // D s_35_8: bits-cast zx s_35_7 -> bv length s_35_6
        let s_35_8: Bits = s_35_7.zero_extend(s_35_6);
        // D s_35_9: read-var t:i64
        let s_35_9: i64 = fn_state.t;
        // D s_35_10: cast zx s_35_9 -> i
        let s_35_10: i128 = (i128::try_from(s_35_9).unwrap());
        // D s_35_11: call X_set(s_35_10, s_35_4, s_35_8)
        let s_35_11: () = X_set(state, tracer, s_35_10, s_35_4, s_35_8);
        // N s_35_12: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var regsizeshadow#1608:i64
        let s_36_0: i64 = fn_state.regsizeshadow_1608;
        // D s_36_1: cast zx s_36_0 -> i
        let s_36_1: i128 = (i128::try_from(s_36_0).unwrap());
        // D s_36_2: call __id(s_36_1)
        let s_36_2: i128 = u__id(state, tracer, s_36_1);
        // D s_36_3: cast reint s_36_2 -> i64
        let s_36_3: i64 = (s_36_2 as i64);
        // D s_36_4: read-var datasizeshadow#1609:i64
        let s_36_4: i64 = fn_state.datasizeshadow_1609;
        // D s_36_5: cast zx s_36_4 -> i
        let s_36_5: i128 = (i128::try_from(s_36_4).unwrap());
        // D s_36_6: call __id(s_36_5)
        let s_36_6: i128 = u__id(state, tracer, s_36_5);
        // D s_36_7: cast reint s_36_6 -> i64
        let s_36_7: i64 = (s_36_6 as i64);
        // D s_36_8: cast zx s_36_3 -> i
        let s_36_8: i128 = (i128::try_from(s_36_3).unwrap());
        // D s_36_9: cast zx s_36_7 -> i
        let s_36_9: i128 = (i128::try_from(s_36_7).unwrap());
        // D s_36_10: cmp-ge s_36_8 s_36_9
        let s_36_10: bool = ((s_36_8) >= (s_36_9));
        // D s_36_11: write-var gs#158639 <= s_36_10
        fn_state.gs_158639 = s_36_10;
        // N s_36_12: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var datasizeshadow#1609:i64
        let s_37_0: i64 = fn_state.datasizeshadow_1609;
        // D s_37_1: cast zx s_37_0 -> i
        let s_37_1: i128 = (i128::try_from(s_37_0).unwrap());
        // D s_37_2: call __id(s_37_1)
        let s_37_2: i128 = u__id(state, tracer, s_37_1);
        // D s_37_3: cast reint s_37_2 -> i64
        let s_37_3: i64 = (s_37_2 as i64);
        // C s_37_4: const #0s : i
        let s_37_4: i128 = 0;
        // D s_37_5: cast zx s_37_3 -> i
        let s_37_5: i128 = (i128::try_from(s_37_3).unwrap());
        // D s_37_6: cmp-gt s_37_5 s_37_4
        let s_37_6: bool = ((s_37_5) > (s_37_4));
        // N s_37_7: branch s_37_6 b40 b38
        if s_37_6 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #0u : u8
        let s_38_0: bool = false;
        // D s_38_1: write-var gs#158642 <= s_38_0
        fn_state.gs_158642 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#158642:u8
        let s_39_0: bool = fn_state.gs_158642;
        // N s_39_1: assert s_39_0
        let s_39_1: () = assert!(s_39_0);
        // D s_39_2: read-var regsizeshadow#1608:i64
        let s_39_2: i64 = fn_state.regsizeshadow_1608;
        // D s_39_3: cast zx s_39_2 -> i
        let s_39_3: i128 = (i128::try_from(s_39_2).unwrap());
        // D s_39_4: cast reint s_39_3 -> i64
        let s_39_4: i64 = (s_39_3 as i64);
        // D s_39_5: read-var regsizeshadow#1608:i64
        let s_39_5: i64 = fn_state.regsizeshadow_1608;
        // D s_39_6: cast zx s_39_5 -> i
        let s_39_6: i128 = (i128::try_from(s_39_5).unwrap());
        // D s_39_7: read-var datashadow#1610:bv
        let s_39_7: Bits = fn_state.datashadow_1610;
        // D s_39_8: bits-cast sx s_39_7 -> bv length s_39_6
        let s_39_8: Bits = s_39_7.sign_extend(s_39_6);
        // D s_39_9: read-var t:i64
        let s_39_9: i64 = fn_state.t;
        // D s_39_10: cast zx s_39_9 -> i
        let s_39_10: i128 = (i128::try_from(s_39_9).unwrap());
        // D s_39_11: call X_set(s_39_10, s_39_4, s_39_8)
        let s_39_11: () = X_set(state, tracer, s_39_10, s_39_4, s_39_8);
        // N s_39_12: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var regsizeshadow#1608:i64
        let s_40_0: i64 = fn_state.regsizeshadow_1608;
        // D s_40_1: cast zx s_40_0 -> i
        let s_40_1: i128 = (i128::try_from(s_40_0).unwrap());
        // D s_40_2: call __id(s_40_1)
        let s_40_2: i128 = u__id(state, tracer, s_40_1);
        // D s_40_3: cast reint s_40_2 -> i64
        let s_40_3: i64 = (s_40_2 as i64);
        // D s_40_4: read-var datasizeshadow#1609:i64
        let s_40_4: i64 = fn_state.datasizeshadow_1609;
        // D s_40_5: cast zx s_40_4 -> i
        let s_40_5: i128 = (i128::try_from(s_40_4).unwrap());
        // D s_40_6: call __id(s_40_5)
        let s_40_6: i128 = u__id(state, tracer, s_40_5);
        // D s_40_7: cast reint s_40_6 -> i64
        let s_40_7: i64 = (s_40_6 as i64);
        // D s_40_8: cast zx s_40_3 -> i
        let s_40_8: i128 = (i128::try_from(s_40_3).unwrap());
        // D s_40_9: cast zx s_40_7 -> i
        let s_40_9: i128 = (i128::try_from(s_40_7).unwrap());
        // D s_40_10: cmp-ge s_40_8 s_40_9
        let s_40_10: bool = ((s_40_8) >= (s_40_9));
        // D s_40_11: write-var gs#158642 <= s_40_10
        fn_state.gs_158642 = s_40_10;
        // N s_40_12: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #8s : i
        let s_41_0: i128 = 8;
        // D s_41_1: read-var datasizeshadow#1609:i64
        let s_41_1: i64 = fn_state.datasizeshadow_1609;
        // D s_41_2: cast zx s_41_1 -> i
        let s_41_2: i128 = (i128::try_from(s_41_1).unwrap());
        // D s_41_3: div s_41_2 s_41_0
        let s_41_3: i128 = ((s_41_2) / (s_41_0));
        // D s_41_4: cast reint s_41_3 -> i64
        let s_41_4: i64 = (s_41_3 as i64);
        // C s_41_5: const #64s : i
        let s_41_5: i128 = 64;
        // D s_41_6: read-var regsizeshadow#1608:i64
        let s_41_6: i64 = fn_state.regsizeshadow_1608;
        // D s_41_7: cast zx s_41_6 -> i
        let s_41_7: i128 = (i128::try_from(s_41_6).unwrap());
        // D s_41_8: cmp-eq s_41_7 s_41_5
        let s_41_8: bool = ((s_41_7) == (s_41_5));
        // D s_41_9: cast zx s_41_4 -> i
        let s_41_9: i128 = (i128::try_from(s_41_4).unwrap());
        // D s_41_10: read-var t:i64
        let s_41_10: i64 = fn_state.t;
        // D s_41_11: cast zx s_41_10 -> i
        let s_41_11: i128 = (i128::try_from(s_41_10).unwrap());
        // D s_41_12: read-var is_signed:u8
        let s_41_12: bool = fn_state.is_signed;
        // C s_41_13: const #0u : u8
        let s_41_13: bool = false;
        // D s_41_14: call AArch64_SetLSInstructionSyndrome(s_41_9, s_41_12, s_41_11, s_41_8, s_41_13)
        let s_41_14: () = AArch64_SetLSInstructionSyndrome(
            state,
            tracer,
            s_41_9,
            s_41_12,
            s_41_11,
            s_41_8,
            s_41_13,
        );
        // N s_41_15: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #2u : u32
        let s_42_0: u32 = 2;
        // D s_42_1: read-var memop:u32
        let s_42_1: u32 = fn_state.memop;
        // D s_42_2: cmp-eq s_42_0 s_42_1
        let s_42_2: bool = ((s_42_0) == (s_42_1));
        // D s_42_3: not s_42_2
        let s_42_3: bool = !s_42_2;
        // N s_42_4: branch s_42_3 b44 b43
        if s_42_3 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #4s : i
        let s_43_0: i128 = 4;
        // C s_43_1: const #0s : i
        let s_43_1: i128 = 0;
        // D s_43_2: read-var t:i64
        let s_43_2: i64 = fn_state.t;
        // D s_43_3: cast zx s_43_2 -> i
        let s_43_3: i128 = (i128::try_from(s_43_2).unwrap());
        // D s_43_4: call integer_subrange(s_43_3, s_43_0, s_43_1)
        let s_43_4: Bits = integer_subrange(state, tracer, s_43_3, s_43_0, s_43_1);
        // D s_43_5: cast reint s_43_4 -> u8
        let s_43_5: u8 = (s_43_4.value() as u8);
        // D s_43_6: read-var address:u64
        let s_43_6: u64 = fn_state.address;
        // D s_43_7: call Prefetch(s_43_6, s_43_5)
        let s_43_7: () = Prefetch(state, tracer, s_43_6, s_43_5);
        // N s_43_8: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_44_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var address:u64
        let s_45_0: u64 = fn_state.address;
        // D s_45_1: cast zx s_45_0 -> bv
        let s_45_1: Bits = Bits::new(s_45_0 as u128, 64u16);
        // D s_45_2: read-var offset:u64
        let s_45_2: u64 = fn_state.offset;
        // D s_45_3: cast zx s_45_2 -> bv
        let s_45_3: Bits = Bits::new(s_45_2 as u128, 64u16);
        // D s_45_4: add s_45_1 s_45_3
        let s_45_4: Bits = (s_45_1 + s_45_3);
        // D s_45_5: cast reint s_45_4 -> u64
        let s_45_5: u64 = (s_45_4.value() as u64);
        // D s_45_6: write-var address <= s_45_5
        fn_state.address = s_45_5;
        // N s_45_7: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var memop:u32
        let s_46_0: u32 = fn_state.memop;
        // C s_46_1: const #2u : u32
        let s_46_1: u32 = 2;
        // D s_46_2: cmp-eq s_46_0 s_46_1
        let s_46_2: bool = ((s_46_0) == (s_46_1));
        // N s_46_3: branch s_46_2 b49 b47
        if s_46_2 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_47_0: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #() : ()
        let s_48_0: () = ();
        // S s_48_1: call SP_read(s_48_0)
        let s_48_1: u64 = SP_read(state, tracer, s_48_0);
        // D s_48_2: write-var address <= s_48_1
        fn_state.address = s_48_1;
        // N s_48_3: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #() : ()
        let s_49_0: () = ();
        // S s_49_1: call CheckSPAlignment(s_49_0)
        let s_49_1: () = CheckSPAlignment(state, tracer, s_49_0);
        // N s_49_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #1u : u32
        let s_50_0: u32 = 1;
        // D s_50_1: read-var tagchecked:u8
        let s_50_1: bool = fn_state.tagchecked;
        // D s_50_2: call CreateAccDescAcqRel(s_50_0, s_50_1)
        let s_50_2: ProductType9878976b5bcce9c9 = CreateAccDescAcqRel(
            state,
            tracer,
            s_50_0,
            s_50_1,
        );
        // D s_50_3: write-var accdesc <= s_50_2
        fn_state.accdesc = s_50_2;
        // N s_50_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var tagchecked:u8
        let s_51_0: bool = fn_state.tagchecked;
        // D s_51_1: call CreateAccDescLDAcqPC(s_51_0)
        let s_51_1: ProductType9878976b5bcce9c9 = CreateAccDescLDAcqPC(
            state,
            tracer,
            s_51_0,
        );
        // D s_51_2: write-var accdesc <= s_51_1
        fn_state.accdesc = s_51_1;
        // N s_51_3: jump b3
        return block_3(state, tracer, fn_state);
    }
}
