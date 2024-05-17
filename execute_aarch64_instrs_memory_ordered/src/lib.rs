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
use Mem_set::*;
use u__UNKNOWN_bits::*;
use u__id::*;
use SP_set::*;
use Mem_read::*;
use CreateAccDescAcqRel::*;
use SP_read::*;
use X_read::*;
use AArch64_SetLSInstructionSyndrome::*;
use CheckSPAlignment::*;
use CreateAccDescLOR::*;
use SPESampleExtendedLoadStore::*;
use common::*;
pub fn execute_aarch64_instrs_memory_ordered<T: Tracer>(
    state: &mut State,
    tracer: &T,
    datasize: i64,
    limitedordered: bool,
    memop: u32,
    n: i64,
    offset: i128,
    regsize: i64,
    rt_unknown: bool,
    t: i64,
    tagchecked: bool,
    wback: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        addressshadow_1625: u64,
        data: Bits,
        address: u64,
        dbytes: i64,
        accdesc: ProductType9878976b5bcce9c9,
        datashadow_1626: Bits,
        regsizeshadow_1623: i64,
        gs_159115: bool,
        datasizeshadow_1624: i64,
        datasize: i64,
        limitedordered: bool,
        memop: u32,
        n: i64,
        offset: i128,
        regsize: i64,
        rt_unknown: bool,
        t: i64,
        tagchecked: bool,
        wback: bool,
    }
    let fn_state = FunctionState {
        datasize,
        limitedordered,
        memop,
        n,
        offset,
        regsize,
        rt_unknown,
        t,
        tagchecked,
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
        // D s_0_3: write-var regsizeshadow#1623 <= s_0_2
        fn_state.regsizeshadow_1623 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1624 <= s_0_6
        fn_state.datasizeshadow_1624 = s_0_6;
        // C s_0_8: const #8s : i
        let s_0_8: i128 = 8;
        // D s_0_9: read-var datasizeshadow#1624:i64
        let s_0_9: i64 = fn_state.datasizeshadow_1624;
        // D s_0_10: cast zx s_0_9 -> i
        let s_0_10: i128 = (i128::try_from(s_0_9).unwrap());
        // D s_0_11: div s_0_10 s_0_8
        let s_0_11: i128 = ((s_0_10) / (s_0_8));
        // D s_0_12: cast reint s_0_11 -> i64
        let s_0_12: i64 = (s_0_11 as i64);
        // D s_0_13: write-var dbytes <= s_0_12
        fn_state.dbytes = s_0_12;
        // D s_0_14: read-var limitedordered:u8
        let s_0_14: bool = fn_state.limitedordered;
        // N s_0_15: branch s_0_14 b27 b1
        if s_0_14 {
            return block_27(state, tracer, fn_state);
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
        // D s_1_1: read-var tagchecked:u8
        let s_1_1: bool = fn_state.tagchecked;
        // D s_1_2: call CreateAccDescAcqRel(s_1_0, s_1_1)
        let s_1_2: ProductType9878976b5bcce9c9 = CreateAccDescAcqRel(
            state,
            tracer,
            s_1_0,
            s_1_1,
        );
        // D s_1_3: write-var accdesc <= s_1_2
        fn_state.accdesc = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #31s : i
        let s_2_0: i128 = 31;
        // D s_2_1: read-var n:i64
        let s_2_1: i64 = fn_state.n;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: cmp-eq s_2_2 s_2_0
        let s_2_3: bool = ((s_2_2) == (s_2_0));
        // N s_2_4: branch s_2_3 b25 b3
        if s_2_3 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #64s : i64
        let s_3_0: i64 = 64;
        // D s_3_1: read-var n:i64
        let s_3_1: i64 = fn_state.n;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: call X_read(s_3_2, s_3_0)
        let s_3_3: Bits = X_read(state, tracer, s_3_2, s_3_0);
        // D s_3_4: cast reint s_3_3 -> u64
        let s_3_4: u64 = (s_3_3.value() as u64);
        // D s_3_5: write-var address <= s_3_4
        fn_state.address = s_3_4;
        // N s_3_6: jump b4
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
        // N s_4_4: branch s_4_3 b18 b5
        if s_4_3 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var address:u64
        let s_5_0: u64 = fn_state.address;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 64u16);
        // D s_5_2: read-var offset:i
        let s_5_2: i128 = fn_state.offset;
        // D s_5_3: cast cvt s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 128);
        // D s_5_4: add s_5_1 s_5_3
        let s_5_4: Bits = (s_5_1 + s_5_3);
        // D s_5_5: cast reint s_5_4 -> u64
        let s_5_5: u64 = (s_5_4.value() as u64);
        // D s_5_6: write-var addressshadow#1625 <= s_5_5
        fn_state.addressshadow_1625 = s_5_5;
        // D s_5_7: read-var rt_unknown:u8
        let s_5_7: bool = fn_state.rt_unknown;
        // N s_5_8: branch s_5_7 b17 b6
        if s_5_7 {
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
        // D s_6_0: read-var datasizeshadow#1624:i64
        let s_6_0: i64 = fn_state.datasizeshadow_1624;
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
        // C s_7_0: const #64s : i
        let s_7_0: i128 = 64;
        // D s_7_1: read-var regsizeshadow#1623:i64
        let s_7_1: i64 = fn_state.regsizeshadow_1623;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: cmp-eq s_7_2 s_7_0
        let s_7_3: bool = ((s_7_2) == (s_7_0));
        // D s_7_4: read-var dbytes:i64
        let s_7_4: i64 = fn_state.dbytes;
        // D s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_6: read-var t:i64
        let s_7_6: i64 = fn_state.t;
        // D s_7_7: cast zx s_7_6 -> i
        let s_7_7: i128 = (i128::try_from(s_7_6).unwrap());
        // C s_7_8: const #0u : u8
        let s_7_8: bool = false;
        // C s_7_9: const #1u : u8
        let s_7_9: bool = true;
        // D s_7_10: call AArch64_SetLSInstructionSyndrome(s_7_5, s_7_8, s_7_7, s_7_3, s_7_9)
        let s_7_10: () = AArch64_SetLSInstructionSyndrome(
            state,
            tracer,
            s_7_5,
            s_7_8,
            s_7_7,
            s_7_3,
            s_7_9,
        );
        // D s_7_11: read-var dbytes:i64
        let s_7_11: i64 = fn_state.dbytes;
        // D s_7_12: cast zx s_7_11 -> i
        let s_7_12: i128 = (i128::try_from(s_7_11).unwrap());
        // D s_7_13: read-var addressshadow#1625:u64
        let s_7_13: u64 = fn_state.addressshadow_1625;
        // D s_7_14: read-var accdesc:struct
        let s_7_14: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_7_15: read-var data:bv
        let s_7_15: Bits = fn_state.data;
        // D s_7_16: call Mem_set(s_7_13, s_7_12, s_7_14, s_7_15)
        let s_7_16: () = Mem_set(state, tracer, s_7_13, s_7_12, s_7_14, s_7_15);
        // N s_7_17: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var wback:u8
        let s_8_0: bool = fn_state.wback;
        // N s_8_1: branch s_8_0 b14 b9
        if s_8_0 {
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
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
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
        // C s_11_0: const #22416u : u32
        let s_11_0: u32 = 22416;
        // D s_11_1: read-reg s_11_0:u8
        let s_11_1: bool = {
            let value = state.read_register::<bool>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // N s_11_2: branch s_11_1 b13 b12
        if s_11_1 {
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
        // D s_13_0: read-var memop:u32
        let s_13_0: u32 = fn_state.memop;
        // C s_13_1: const #0u : u32
        let s_13_1: u32 = 0;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // C s_13_3: const #1u : u8
        let s_13_3: bool = true;
        // C s_13_4: const #0u : u8
        let s_13_4: bool = false;
        // C s_13_5: const #0u : u8
        let s_13_5: bool = false;
        // D s_13_6: call SPESampleExtendedLoadStore(s_13_3, s_13_4, s_13_5, s_13_2)
        let s_13_6: () = SPESampleExtendedLoadStore(
            state,
            tracer,
            s_13_3,
            s_13_4,
            s_13_5,
            s_13_2,
        );
        // N s_13_7: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #31s : i
        let s_14_0: i128 = 31;
        // D s_14_1: read-var n:i64
        let s_14_1: i64 = fn_state.n;
        // D s_14_2: cast zx s_14_1 -> i
        let s_14_2: i128 = (i128::try_from(s_14_1).unwrap());
        // D s_14_3: cmp-eq s_14_2 s_14_0
        let s_14_3: bool = ((s_14_2) == (s_14_0));
        // N s_14_4: branch s_14_3 b16 b15
        if s_14_3 {
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
        // C s_15_0: const #64s : i64
        let s_15_0: i64 = 64;
        // D s_15_1: read-var n:i64
        let s_15_1: i64 = fn_state.n;
        // D s_15_2: cast zx s_15_1 -> i
        let s_15_2: i128 = (i128::try_from(s_15_1).unwrap());
        // D s_15_3: read-var addressshadow#1625:u64
        let s_15_3: u64 = fn_state.addressshadow_1625;
        // D s_15_4: cast zx s_15_3 -> bv
        let s_15_4: Bits = Bits::new(s_15_3 as u128, 64u16);
        // D s_15_5: call X_set(s_15_2, s_15_0, s_15_4)
        let s_15_5: () = X_set(state, tracer, s_15_2, s_15_0, s_15_4);
        // N s_15_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var addressshadow#1625:u64
        let s_16_0: u64 = fn_state.addressshadow_1625;
        // D s_16_1: call SP_set(s_16_0)
        let s_16_1: () = SP_set(state, tracer, s_16_0);
        // N s_16_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var datasizeshadow#1624:i64
        let s_17_0: i64 = fn_state.datasizeshadow_1624;
        // D s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_2: cast reint s_17_1 -> i64
        let s_17_2: i64 = (s_17_1 as i64);
        // D s_17_3: cast zx s_17_2 -> i
        let s_17_3: i128 = (i128::try_from(s_17_2).unwrap());
        // D s_17_4: call __UNKNOWN_bits(s_17_3)
        let s_17_4: Bits = u__UNKNOWN_bits(state, tracer, s_17_3);
        // D s_17_5: write-var data <= s_17_4
        fn_state.data = s_17_4;
        // N s_17_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0u : u32
        let s_18_0: u32 = 0;
        // D s_18_1: read-var memop:u32
        let s_18_1: u32 = fn_state.memop;
        // D s_18_2: cmp-eq s_18_0 s_18_1
        let s_18_2: bool = ((s_18_0) == (s_18_1));
        // D s_18_3: not s_18_2
        let s_18_3: bool = !s_18_2;
        // N s_18_4: branch s_18_3 b24 b19
        if s_18_3 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #64s : i
        let s_19_0: i128 = 64;
        // D s_19_1: read-var regsizeshadow#1623:i64
        let s_19_1: i64 = fn_state.regsizeshadow_1623;
        // D s_19_2: cast zx s_19_1 -> i
        let s_19_2: i128 = (i128::try_from(s_19_1).unwrap());
        // D s_19_3: cmp-eq s_19_2 s_19_0
        let s_19_3: bool = ((s_19_2) == (s_19_0));
        // D s_19_4: read-var dbytes:i64
        let s_19_4: i64 = fn_state.dbytes;
        // D s_19_5: cast zx s_19_4 -> i
        let s_19_5: i128 = (i128::try_from(s_19_4).unwrap());
        // D s_19_6: read-var t:i64
        let s_19_6: i64 = fn_state.t;
        // D s_19_7: cast zx s_19_6 -> i
        let s_19_7: i128 = (i128::try_from(s_19_6).unwrap());
        // C s_19_8: const #0u : u8
        let s_19_8: bool = false;
        // C s_19_9: const #1u : u8
        let s_19_9: bool = true;
        // D s_19_10: call AArch64_SetLSInstructionSyndrome(s_19_5, s_19_8, s_19_7, s_19_3, s_19_9)
        let s_19_10: () = AArch64_SetLSInstructionSyndrome(
            state,
            tracer,
            s_19_5,
            s_19_8,
            s_19_7,
            s_19_3,
            s_19_9,
        );
        // D s_19_11: read-var dbytes:i64
        let s_19_11: i64 = fn_state.dbytes;
        // D s_19_12: cast zx s_19_11 -> i
        let s_19_12: i128 = (i128::try_from(s_19_11).unwrap());
        // D s_19_13: read-var address:u64
        let s_19_13: u64 = fn_state.address;
        // D s_19_14: read-var accdesc:struct
        let s_19_14: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_19_15: call Mem_read(s_19_13, s_19_12, s_19_14)
        let s_19_15: Bits = Mem_read(state, tracer, s_19_13, s_19_12, s_19_14);
        // D s_19_16: write-var datashadow#1626 <= s_19_15
        fn_state.datashadow_1626 = s_19_15;
        // N s_19_17: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var datasizeshadow#1624:i64
        let s_20_0: i64 = fn_state.datasizeshadow_1624;
        // D s_20_1: cast zx s_20_0 -> i
        let s_20_1: i128 = (i128::try_from(s_20_0).unwrap());
        // D s_20_2: call __id(s_20_1)
        let s_20_2: i128 = u__id(state, tracer, s_20_1);
        // D s_20_3: cast reint s_20_2 -> i64
        let s_20_3: i64 = (s_20_2 as i64);
        // C s_20_4: const #0s : i
        let s_20_4: i128 = 0;
        // D s_20_5: cast zx s_20_3 -> i
        let s_20_5: i128 = (i128::try_from(s_20_3).unwrap());
        // D s_20_6: cmp-ge s_20_5 s_20_4
        let s_20_6: bool = ((s_20_5) >= (s_20_4));
        // N s_20_7: branch s_20_6 b23 b21
        if s_20_6 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#159115 <= s_21_0
        fn_state.gs_159115 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#159115:u8
        let s_22_0: bool = fn_state.gs_159115;
        // N s_22_1: assert s_22_0
        let s_22_1: () = assert!(s_22_0);
        // D s_22_2: read-var regsizeshadow#1623:i64
        let s_22_2: i64 = fn_state.regsizeshadow_1623;
        // D s_22_3: cast zx s_22_2 -> i
        let s_22_3: i128 = (i128::try_from(s_22_2).unwrap());
        // D s_22_4: cast reint s_22_3 -> i64
        let s_22_4: i64 = (s_22_3 as i64);
        // D s_22_5: read-var regsizeshadow#1623:i64
        let s_22_5: i64 = fn_state.regsizeshadow_1623;
        // D s_22_6: cast zx s_22_5 -> i
        let s_22_6: i128 = (i128::try_from(s_22_5).unwrap());
        // D s_22_7: read-var datashadow#1626:bv
        let s_22_7: Bits = fn_state.datashadow_1626;
        // D s_22_8: bits-cast zx s_22_7 -> bv length s_22_6
        let s_22_8: Bits = s_22_7.zero_extend(s_22_6);
        // D s_22_9: read-var t:i64
        let s_22_9: i64 = fn_state.t;
        // D s_22_10: cast zx s_22_9 -> i
        let s_22_10: i128 = (i128::try_from(s_22_9).unwrap());
        // D s_22_11: call X_set(s_22_10, s_22_4, s_22_8)
        let s_22_11: () = X_set(state, tracer, s_22_10, s_22_4, s_22_8);
        // N s_22_12: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var regsizeshadow#1623:i64
        let s_23_0: i64 = fn_state.regsizeshadow_1623;
        // D s_23_1: cast zx s_23_0 -> i
        let s_23_1: i128 = (i128::try_from(s_23_0).unwrap());
        // D s_23_2: call __id(s_23_1)
        let s_23_2: i128 = u__id(state, tracer, s_23_1);
        // D s_23_3: cast reint s_23_2 -> i64
        let s_23_3: i64 = (s_23_2 as i64);
        // D s_23_4: read-var datasizeshadow#1624:i64
        let s_23_4: i64 = fn_state.datasizeshadow_1624;
        // D s_23_5: cast zx s_23_4 -> i
        let s_23_5: i128 = (i128::try_from(s_23_4).unwrap());
        // D s_23_6: call __id(s_23_5)
        let s_23_6: i128 = u__id(state, tracer, s_23_5);
        // D s_23_7: cast reint s_23_6 -> i64
        let s_23_7: i64 = (s_23_6 as i64);
        // D s_23_8: cast zx s_23_3 -> i
        let s_23_8: i128 = (i128::try_from(s_23_3).unwrap());
        // D s_23_9: cast zx s_23_7 -> i
        let s_23_9: i128 = (i128::try_from(s_23_7).unwrap());
        // D s_23_10: cmp-ge s_23_8 s_23_9
        let s_23_10: bool = ((s_23_8) >= (s_23_9));
        // D s_23_11: write-var gs#159115 <= s_23_10
        fn_state.gs_159115 = s_23_10;
        // N s_23_12: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call CheckSPAlignment(s_25_0)
        let s_25_1: () = CheckSPAlignment(state, tracer, s_25_0);
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #() : ()
        let s_26_0: () = ();
        // S s_26_1: call SP_read(s_26_0)
        let s_26_1: u64 = SP_read(state, tracer, s_26_0);
        // D s_26_2: write-var address <= s_26_1
        fn_state.address = s_26_1;
        // N s_26_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var memop:u32
        let s_27_0: u32 = fn_state.memop;
        // D s_27_1: read-var tagchecked:u8
        let s_27_1: bool = fn_state.tagchecked;
        // D s_27_2: call CreateAccDescLOR(s_27_0, s_27_1)
        let s_27_2: ProductType9878976b5bcce9c9 = CreateAccDescLOR(
            state,
            tracer,
            s_27_0,
            s_27_1,
        );
        // D s_27_3: write-var accdesc <= s_27_2
        fn_state.accdesc = s_27_2;
        // N s_27_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
