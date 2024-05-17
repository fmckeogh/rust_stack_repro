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
use V_read::*;
use SP_set::*;
use fmod_int::*;
use X_read::*;
use Elem_read::*;
use CheckFPAdvSIMDEnabled64::*;
use neq_int::*;
use X_set::*;
use Elem_set::*;
use V_set::*;
use Mem_read::*;
use Mem_set::*;
use SP_read::*;
use replicate_bits_borealis_internal::*;
use SPESampleSIMDFPLoadStore::*;
use CreateAccDescASIMD::*;
use CheckSPAlignment::*;
use common::*;
pub fn execute_aarch64_instrs_memory_vector_single_no_wb<T: Tracer>(
    state: &mut State,
    tracer: &T,
    datasize: i64,
    esize: i64,
    index: i128,
    m: i128,
    memop: u32,
    n: i64,
    nontemporal: bool,
    replicate: bool,
    selem: i64,
    t__arg: i64,
    tagchecked: bool,
    wback: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        t: i128,
        ebytes: i64,
        ga_256940: Bits,
        address: u64,
        offs: u64,
        u_2024: i64,
        datasizeshadow_1489: i64,
        rval: u128,
        gs_155208: i64,
        s: i64,
        ga_256939: i64,
        accdesc: ProductType9878976b5bcce9c9,
        element: Bits,
        esizeshadow_1488: i64,
        gs_155223: i64,
        datasize: i64,
        esize: i64,
        index: i128,
        m: i128,
        memop: u32,
        n: i64,
        nontemporal: bool,
        replicate: bool,
        selem: i64,
        t__arg: i64,
        tagchecked: bool,
        wback: bool,
    }
    let fn_state = FunctionState {
        datasize,
        esize,
        index,
        m,
        memop,
        n,
        nontemporal,
        replicate,
        selem,
        t__arg,
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
        // D s_0_0: read-var esize:i64
        let s_0_0: i64 = fn_state.esize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var esizeshadow#1488 <= s_0_2
        fn_state.esizeshadow_1488 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1489 <= s_0_6
        fn_state.datasizeshadow_1489 = s_0_6;
        // D s_0_8: read-var t__arg:i64
        let s_0_8: i64 = fn_state.t__arg;
        // D s_0_9: cast zx s_0_8 -> i
        let s_0_9: i128 = (i128::try_from(s_0_8).unwrap());
        // D s_0_10: write-var t <= s_0_9
        fn_state.t = s_0_9;
        // C s_0_11: const #() : ()
        let s_0_11: () = ();
        // S s_0_12: call CheckFPAdvSIMDEnabled64(s_0_11)
        let s_0_12: () = CheckFPAdvSIMDEnabled64(state, tracer, s_0_11);
        // N s_0_13: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #8s : i
        let s_1_0: i128 = 8;
        // D s_1_1: read-var esizeshadow#1488:i64
        let s_1_1: i64 = fn_state.esizeshadow_1488;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: div s_1_2 s_1_0
        let s_1_3: i128 = ((s_1_2) / (s_1_0));
        // D s_1_4: cast reint s_1_3 -> i64
        let s_1_4: i64 = (s_1_3 as i64);
        // D s_1_5: write-var ebytes <= s_1_4
        fn_state.ebytes = s_1_4;
        // D s_1_6: read-var memop:u32
        let s_1_6: u32 = fn_state.memop;
        // D s_1_7: read-var nontemporal:u8
        let s_1_7: bool = fn_state.nontemporal;
        // D s_1_8: read-var tagchecked:u8
        let s_1_8: bool = fn_state.tagchecked;
        // D s_1_9: call CreateAccDescASIMD(s_1_6, s_1_7, s_1_8)
        let s_1_9: ProductType9878976b5bcce9c9 = CreateAccDescASIMD(
            state,
            tracer,
            s_1_6,
            s_1_7,
            s_1_8,
        );
        // D s_1_10: write-var accdesc <= s_1_9
        fn_state.accdesc = s_1_9;
        // C s_1_11: const #31s : i
        let s_1_11: i128 = 31;
        // D s_1_12: read-var n:i64
        let s_1_12: i64 = fn_state.n;
        // D s_1_13: cast zx s_1_12 -> i
        let s_1_13: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_14: cmp-eq s_1_13 s_1_11
        let s_1_14: bool = ((s_1_13) == (s_1_11));
        // N s_1_15: branch s_1_14 b28 b2
        if s_1_14 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #64s : i64
        let s_2_0: i64 = 64;
        // D s_2_1: read-var n:i64
        let s_2_1: i64 = fn_state.n;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: call X_read(s_2_2, s_2_0)
        let s_2_3: Bits = X_read(state, tracer, s_2_2, s_2_0);
        // D s_2_4: cast reint s_2_3 -> u64
        let s_2_4: u64 = (s_2_3.value() as u64);
        // D s_2_5: write-var address <= s_2_4
        fn_state.address = s_2_4;
        // N s_2_6: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u64
        let s_3_0: u64 = 0;
        // D s_3_1: write-var offs <= s_3_0
        fn_state.offs = s_3_0;
        // D s_3_2: read-var replicate:u8
        let s_3_2: bool = fn_state.replicate;
        // N s_3_3: branch s_3_2 b23 b4
        if s_3_2 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0s : i64
        let s_4_0: i64 = 0;
        // C s_4_1: const #1s : i
        let s_4_1: i128 = 1;
        // D s_4_2: read-var selem:i64
        let s_4_2: i64 = fn_state.selem;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: sub s_4_3 s_4_1
        let s_4_4: i128 = ((s_4_3) - (s_4_1));
        // D s_4_5: cast reint s_4_4 -> i64
        let s_4_5: i64 = (s_4_4 as i64);
        // D s_4_6: write-var gs#155208 <= s_4_5
        fn_state.gs_155208 = s_4_5;
        // D s_4_7: write-var u#2024 <= s_4_0
        fn_state.u_2024 = s_4_0;
        // N s_4_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var u#2024:i64
        let s_5_0: i64 = fn_state.u_2024;
        // D s_5_1: read-var gs#155208:i64
        let s_5_1: i64 = fn_state.gs_155208;
        // D s_5_2: cmp-gt s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) > (s_5_1));
        // N s_5_3: branch s_5_2 b11 b6
        if s_5_2 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #128s : i64
        let s_6_0: i64 = 128;
        // D s_6_1: read-var t:i
        let s_6_1: i128 = fn_state.t;
        // D s_6_2: call V_read(s_6_1, s_6_0)
        let s_6_2: Bits = V_read(state, tracer, s_6_1, s_6_0);
        // D s_6_3: cast reint s_6_2 -> u128
        let s_6_3: u128 = (s_6_2.value() as u128);
        // D s_6_4: write-var rval <= s_6_3
        fn_state.rval = s_6_3;
        // D s_6_5: read-var memop:u32
        let s_6_5: u32 = fn_state.memop;
        // C s_6_6: const #0u : u32
        let s_6_6: u32 = 0;
        // D s_6_7: cmp-eq s_6_5 s_6_6
        let s_6_7: bool = ((s_6_5) == (s_6_6));
        // N s_6_8: branch s_6_7 b9 b7
        if s_6_7 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var address:u64
        let s_7_0: u64 = fn_state.address;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 64u16);
        // D s_7_2: read-var offs:u64
        let s_7_2: u64 = fn_state.offs;
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 64u16);
        // D s_7_4: add s_7_1 s_7_3
        let s_7_4: Bits = (s_7_1 + s_7_3);
        // D s_7_5: cast reint s_7_4 -> u64
        let s_7_5: u64 = (s_7_4.value() as u64);
        // D s_7_6: read-var esizeshadow#1488:i64
        let s_7_6: i64 = fn_state.esizeshadow_1488;
        // D s_7_7: cast zx s_7_6 -> i
        let s_7_7: i128 = (i128::try_from(s_7_6).unwrap());
        // D s_7_8: cast reint s_7_7 -> i64
        let s_7_8: i64 = (s_7_7 as i64);
        // D s_7_9: read-var rval:u128
        let s_7_9: u128 = fn_state.rval;
        // D s_7_10: cast zx s_7_9 -> bv
        let s_7_10: Bits = Bits::new(s_7_9 as u128, 128u16);
        // D s_7_11: cast zx s_7_8 -> i
        let s_7_11: i128 = (i128::try_from(s_7_8).unwrap());
        // D s_7_12: read-var index:i
        let s_7_12: i128 = fn_state.index;
        // D s_7_13: call Elem_read(s_7_10, s_7_12, s_7_11)
        let s_7_13: Bits = Elem_read(state, tracer, s_7_10, s_7_12, s_7_11);
        // D s_7_14: read-var ebytes:i64
        let s_7_14: i64 = fn_state.ebytes;
        // D s_7_15: cast zx s_7_14 -> i
        let s_7_15: i128 = (i128::try_from(s_7_14).unwrap());
        // D s_7_16: read-var accdesc:struct
        let s_7_16: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_7_17: call Mem_set(s_7_5, s_7_15, s_7_16, s_7_13)
        let s_7_17: () = Mem_set(state, tracer, s_7_5, s_7_15, s_7_16, s_7_13);
        // N s_7_18: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var offs:u64
        let s_8_0: u64 = fn_state.offs;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 64u16);
        // D s_8_2: read-var ebytes:i64
        let s_8_2: i64 = fn_state.ebytes;
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_4: cast cvt s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 128);
        // D s_8_5: add s_8_1 s_8_4
        let s_8_5: Bits = (s_8_1 + s_8_4);
        // D s_8_6: cast reint s_8_5 -> u64
        let s_8_6: u64 = (s_8_5.value() as u64);
        // D s_8_7: write-var offs <= s_8_6
        fn_state.offs = s_8_6;
        // C s_8_8: const #1s : i
        let s_8_8: i128 = 1;
        // D s_8_9: read-var t:i
        let s_8_9: i128 = fn_state.t;
        // D s_8_10: add s_8_9 s_8_8
        let s_8_10: i128 = (s_8_9 + s_8_8);
        // C s_8_11: const #32s : i
        let s_8_11: i128 = 32;
        // D s_8_12: call fmod_int(s_8_10, s_8_11)
        let s_8_12: i128 = fmod_int(state, tracer, s_8_10, s_8_11);
        // D s_8_13: write-var t <= s_8_12
        fn_state.t = s_8_12;
        // D s_8_14: read-var u#2024:i64
        let s_8_14: i64 = fn_state.u_2024;
        // C s_8_15: const #1s : i64
        let s_8_15: i64 = 1;
        // D s_8_16: add s_8_14 s_8_15
        let s_8_16: i64 = (s_8_14 + s_8_15);
        // D s_8_17: write-var u#2024 <= s_8_16
        fn_state.u_2024 = s_8_16;
        // N s_8_18: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var esizeshadow#1488:i64
        let s_9_0: i64 = fn_state.esizeshadow_1488;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: cast reint s_9_1 -> i64
        let s_9_2: i64 = (s_9_1 as i64);
        // D s_9_3: write-var ga#256939 <= s_9_2
        fn_state.ga_256939 = s_9_2;
        // D s_9_4: read-var address:u64
        let s_9_4: u64 = fn_state.address;
        // D s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 64u16);
        // D s_9_6: read-var offs:u64
        let s_9_6: u64 = fn_state.offs;
        // D s_9_7: cast zx s_9_6 -> bv
        let s_9_7: Bits = Bits::new(s_9_6 as u128, 64u16);
        // D s_9_8: add s_9_5 s_9_7
        let s_9_8: Bits = (s_9_5 + s_9_7);
        // D s_9_9: cast reint s_9_8 -> u64
        let s_9_9: u64 = (s_9_8.value() as u64);
        // D s_9_10: read-var ebytes:i64
        let s_9_10: i64 = fn_state.ebytes;
        // D s_9_11: cast zx s_9_10 -> i
        let s_9_11: i128 = (i128::try_from(s_9_10).unwrap());
        // D s_9_12: read-var accdesc:struct
        let s_9_12: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_9_13: call Mem_read(s_9_9, s_9_11, s_9_12)
        let s_9_13: Bits = Mem_read(state, tracer, s_9_9, s_9_11, s_9_12);
        // D s_9_14: write-var ga#256940 <= s_9_13
        fn_state.ga_256940 = s_9_13;
        // N s_9_15: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var rval:u128
        let s_10_0: u128 = fn_state.rval;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 128u16);
        // D s_10_2: read-var ga#256939:i64
        let s_10_2: i64 = fn_state.ga_256939;
        // D s_10_3: cast zx s_10_2 -> i
        let s_10_3: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_4: read-var index:i
        let s_10_4: i128 = fn_state.index;
        // D s_10_5: read-var ga#256940:bv
        let s_10_5: Bits = fn_state.ga_256940;
        // D s_10_6: call Elem_set(s_10_1, s_10_4, s_10_3, s_10_5)
        let s_10_6: Bits = Elem_set(state, tracer, s_10_1, s_10_4, s_10_3, s_10_5);
        // D s_10_7: cast reint s_10_6 -> u128
        let s_10_7: u128 = (s_10_6.value() as u128);
        // D s_10_8: write-var rval <= s_10_7
        fn_state.rval = s_10_7;
        // C s_10_9: const #128s : i64
        let s_10_9: i64 = 128;
        // D s_10_10: read-var rval:u128
        let s_10_10: u128 = fn_state.rval;
        // D s_10_11: cast zx s_10_10 -> bv
        let s_10_11: Bits = Bits::new(s_10_10 as u128, 128u16);
        // D s_10_12: read-var t:i
        let s_10_12: i128 = fn_state.t;
        // D s_10_13: call V_set(s_10_12, s_10_9, s_10_11)
        let s_10_13: () = V_set(state, tracer, s_10_12, s_10_9, s_10_11);
        // N s_10_14: jump b8
        return block_8(state, tracer, fn_state);
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
        // N s_12_2: branch s_12_1 b22 b13
        if s_12_1 {
            return block_22(state, tracer, fn_state);
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
        // D s_14_0: read-var wback:u8
        let s_14_0: bool = fn_state.wback;
        // N s_14_1: branch s_14_0 b16 b15
        if s_14_0 {
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
        // C s_16_0: const #31s : i
        let s_16_0: i128 = 31;
        // D s_16_1: read-var m:i
        let s_16_1: i128 = fn_state.m;
        // D s_16_2: call neq_int(s_16_1, s_16_0)
        let s_16_2: bool = neq_int(state, tracer, s_16_1, s_16_0);
        // N s_16_3: branch s_16_2 b21 b17
        if s_16_2 {
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
        // D s_19_1: read-var address:u64
        let s_19_1: u64 = fn_state.address;
        // D s_19_2: cast zx s_19_1 -> bv
        let s_19_2: Bits = Bits::new(s_19_1 as u128, 64u16);
        // D s_19_3: read-var offs:u64
        let s_19_3: u64 = fn_state.offs;
        // D s_19_4: cast zx s_19_3 -> bv
        let s_19_4: Bits = Bits::new(s_19_3 as u128, 64u16);
        // D s_19_5: add s_19_2 s_19_4
        let s_19_5: Bits = (s_19_2 + s_19_4);
        // D s_19_6: cast reint s_19_5 -> u64
        let s_19_6: u64 = (s_19_5.value() as u64);
        // D s_19_7: read-var n:i64
        let s_19_7: i64 = fn_state.n;
        // D s_19_8: cast zx s_19_7 -> i
        let s_19_8: i128 = (i128::try_from(s_19_7).unwrap());
        // D s_19_9: cast zx s_19_6 -> bv
        let s_19_9: Bits = Bits::new(s_19_6 as u128, 64u16);
        // D s_19_10: call X_set(s_19_8, s_19_0, s_19_9)
        let s_19_10: () = X_set(state, tracer, s_19_8, s_19_0, s_19_9);
        // N s_19_11: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var address:u64
        let s_20_0: u64 = fn_state.address;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 64u16);
        // D s_20_2: read-var offs:u64
        let s_20_2: u64 = fn_state.offs;
        // D s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 64u16);
        // D s_20_4: add s_20_1 s_20_3
        let s_20_4: Bits = (s_20_1 + s_20_3);
        // D s_20_5: cast reint s_20_4 -> u64
        let s_20_5: u64 = (s_20_4.value() as u64);
        // D s_20_6: call SP_set(s_20_5)
        let s_20_6: () = SP_set(state, tracer, s_20_5);
        // N s_20_7: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #64s : i64
        let s_21_0: i64 = 64;
        // D s_21_1: read-var m:i
        let s_21_1: i128 = fn_state.m;
        // D s_21_2: call X_read(s_21_1, s_21_0)
        let s_21_2: Bits = X_read(state, tracer, s_21_1, s_21_0);
        // D s_21_3: cast reint s_21_2 -> u64
        let s_21_3: u64 = (s_21_2.value() as u64);
        // D s_21_4: write-var offs <= s_21_3
        fn_state.offs = s_21_3;
        // N s_21_5: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call SPESampleSIMDFPLoadStore(s_22_0)
        let s_22_1: () = SPESampleSIMDFPLoadStore(state, tracer, s_22_0);
        // N s_22_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0s : i64
        let s_23_0: i64 = 0;
        // C s_23_1: const #1s : i
        let s_23_1: i128 = 1;
        // D s_23_2: read-var selem:i64
        let s_23_2: i64 = fn_state.selem;
        // D s_23_3: cast zx s_23_2 -> i
        let s_23_3: i128 = (i128::try_from(s_23_2).unwrap());
        // D s_23_4: sub s_23_3 s_23_1
        let s_23_4: i128 = ((s_23_3) - (s_23_1));
        // D s_23_5: cast reint s_23_4 -> i64
        let s_23_5: i64 = (s_23_4 as i64);
        // D s_23_6: write-var gs#155223 <= s_23_5
        fn_state.gs_155223 = s_23_5;
        // D s_23_7: write-var s <= s_23_0
        fn_state.s = s_23_0;
        // N s_23_8: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var s:i64
        let s_24_0: i64 = fn_state.s;
        // D s_24_1: read-var gs#155223:i64
        let s_24_1: i64 = fn_state.gs_155223;
        // D s_24_2: cmp-gt s_24_0 s_24_1
        let s_24_2: bool = ((s_24_0) > (s_24_1));
        // N s_24_3: branch s_24_2 b27 b25
        if s_24_2 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var address:u64
        let s_25_0: u64 = fn_state.address;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 64u16);
        // D s_25_2: read-var offs:u64
        let s_25_2: u64 = fn_state.offs;
        // D s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 64u16);
        // D s_25_4: add s_25_1 s_25_3
        let s_25_4: Bits = (s_25_1 + s_25_3);
        // D s_25_5: cast reint s_25_4 -> u64
        let s_25_5: u64 = (s_25_4.value() as u64);
        // D s_25_6: read-var ebytes:i64
        let s_25_6: i64 = fn_state.ebytes;
        // D s_25_7: cast zx s_25_6 -> i
        let s_25_7: i128 = (i128::try_from(s_25_6).unwrap());
        // D s_25_8: read-var accdesc:struct
        let s_25_8: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_25_9: call Mem_read(s_25_5, s_25_7, s_25_8)
        let s_25_9: Bits = Mem_read(state, tracer, s_25_5, s_25_7, s_25_8);
        // D s_25_10: write-var element <= s_25_9
        fn_state.element = s_25_9;
        // N s_25_11: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var datasizeshadow#1489:i64
        let s_26_0: i64 = fn_state.datasizeshadow_1489;
        // D s_26_1: cast zx s_26_0 -> i
        let s_26_1: i128 = (i128::try_from(s_26_0).unwrap());
        // D s_26_2: cast reint s_26_1 -> i64
        let s_26_2: i64 = (s_26_1 as i64);
        // D s_26_3: read-var datasizeshadow#1489:i64
        let s_26_3: i64 = fn_state.datasizeshadow_1489;
        // D s_26_4: cast zx s_26_3 -> i
        let s_26_4: i128 = (i128::try_from(s_26_3).unwrap());
        // D s_26_5: read-var esizeshadow#1488:i64
        let s_26_5: i64 = fn_state.esizeshadow_1488;
        // D s_26_6: cast zx s_26_5 -> i
        let s_26_6: i128 = (i128::try_from(s_26_5).unwrap());
        // D s_26_7: div s_26_4 s_26_6
        let s_26_7: i128 = ((s_26_4) / (s_26_6));
        // D s_26_8: cast reint s_26_7 -> i64
        let s_26_8: i64 = (s_26_7 as i64);
        // D s_26_9: cast zx s_26_8 -> i
        let s_26_9: i128 = (i128::try_from(s_26_8).unwrap());
        // D s_26_10: read-var element:bv
        let s_26_10: Bits = fn_state.element;
        // D s_26_11: cast reint s_26_9 -> u64
        let s_26_11: u64 = (s_26_9 as u64);
        // D s_26_12: call replicate_bits_borealis_internal(s_26_10, s_26_11)
        let s_26_12: Bits = replicate_bits_borealis_internal(
            state,
            tracer,
            s_26_10,
            s_26_11,
        );
        // D s_26_13: read-var t:i
        let s_26_13: i128 = fn_state.t;
        // D s_26_14: call V_set(s_26_13, s_26_2, s_26_12)
        let s_26_14: () = V_set(state, tracer, s_26_13, s_26_2, s_26_12);
        // D s_26_15: read-var offs:u64
        let s_26_15: u64 = fn_state.offs;
        // D s_26_16: cast zx s_26_15 -> bv
        let s_26_16: Bits = Bits::new(s_26_15 as u128, 64u16);
        // D s_26_17: read-var ebytes:i64
        let s_26_17: i64 = fn_state.ebytes;
        // D s_26_18: cast zx s_26_17 -> i
        let s_26_18: i128 = (i128::try_from(s_26_17).unwrap());
        // D s_26_19: cast cvt s_26_18 -> bv
        let s_26_19: Bits = Bits::new(s_26_18 as u128, 128);
        // D s_26_20: add s_26_16 s_26_19
        let s_26_20: Bits = (s_26_16 + s_26_19);
        // D s_26_21: cast reint s_26_20 -> u64
        let s_26_21: u64 = (s_26_20.value() as u64);
        // D s_26_22: write-var offs <= s_26_21
        fn_state.offs = s_26_21;
        // C s_26_23: const #1s : i
        let s_26_23: i128 = 1;
        // D s_26_24: read-var t:i
        let s_26_24: i128 = fn_state.t;
        // D s_26_25: add s_26_24 s_26_23
        let s_26_25: i128 = (s_26_24 + s_26_23);
        // C s_26_26: const #32s : i
        let s_26_26: i128 = 32;
        // D s_26_27: call fmod_int(s_26_25, s_26_26)
        let s_26_27: i128 = fmod_int(state, tracer, s_26_25, s_26_26);
        // D s_26_28: write-var t <= s_26_27
        fn_state.t = s_26_27;
        // D s_26_29: read-var s:i64
        let s_26_29: i64 = fn_state.s;
        // C s_26_30: const #1s : i64
        let s_26_30: i64 = 1;
        // D s_26_31: add s_26_29 s_26_30
        let s_26_31: i64 = (s_26_29 + s_26_30);
        // D s_26_32: write-var s <= s_26_31
        fn_state.s = s_26_31;
        // N s_26_33: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call CheckSPAlignment(s_28_0)
        let s_28_1: () = CheckSPAlignment(state, tracer, s_28_0);
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call SP_read(s_29_0)
        let s_29_1: u64 = SP_read(state, tracer, s_29_0);
        // D s_29_2: write-var address <= s_29_1
        fn_state.address = s_29_1;
        // N s_29_3: jump b3
        return block_3(state, tracer, fn_state);
    }
}
