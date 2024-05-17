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
use R_read::*;
use CheckAdvSIMDEnabled::*;
use R_set::*;
use AArch32_Abort::*;
use replicate_bits_borealis_internal::*;
use D_set::*;
use IsAligned__1::*;
use CreateAccDescASIMD::*;
use MemU_read::*;
use AlignmentFault::*;
use common::*;
pub fn execute_aarch32_instrs_VLD1_a_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    alignment: i64,
    d: i64,
    ebytes: i64,
    m: i64,
    n: i64,
    register_index: bool,
    regs: i64,
    wback: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        esize: i64,
        address: u32,
        accdesc: ProductType9878976b5bcce9c9,
        replicated_element: u64,
        element: Bits,
        gs_309427: i64,
        alignment: i64,
        d: i64,
        ebytes: i64,
        m: i64,
        n: i64,
        register_index: bool,
        regs: i64,
        wback: bool,
    }
    let fn_state = FunctionState {
        alignment,
        d,
        ebytes,
        m,
        n,
        register_index,
        regs,
        wback,
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
        // S s_0_1: call CheckAdvSIMDEnabled(s_0_0)
        let s_0_1: () = CheckAdvSIMDEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
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
        // D s_1_3: write-var address <= s_1_2
        fn_state.address = s_1_2;
        // C s_1_4: const #0u : u8
        let s_1_4: bool = false;
        // C s_1_5: const #0u : u8
        let s_1_5: bool = false;
        // C s_1_6: const #0u : u32
        let s_1_6: u32 = 0;
        // S s_1_7: call CreateAccDescASIMD(s_1_6, s_1_4, s_1_5)
        let s_1_7: ProductType9878976b5bcce9c9 = CreateAccDescASIMD(
            state,
            tracer,
            s_1_6,
            s_1_4,
            s_1_5,
        );
        // D s_1_8: write-var accdesc <= s_1_7
        fn_state.accdesc = s_1_7;
        // D s_1_9: read-var address:u32
        let s_1_9: u32 = fn_state.address;
        // D s_1_10: cast zx s_1_9 -> bv
        let s_1_10: Bits = Bits::new(s_1_9 as u128, 32u16);
        // D s_1_11: read-var alignment:i64
        let s_1_11: i64 = fn_state.alignment;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: call IsAligned__1(s_1_10, s_1_12)
        let s_1_13: bool = IsAligned__1(state, tracer, s_1_10, s_1_12);
        // D s_1_14: not s_1_13
        let s_1_14: bool = !s_1_13;
        // N s_1_15: branch s_1_14 b12 b2
        if s_1_14 {
            return block_12(state, tracer, fn_state);
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
        // C s_3_0: const #8s : i
        let s_3_0: i128 = 8;
        // D s_3_1: read-var ebytes:i64
        let s_3_1: i64 = fn_state.ebytes;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: mul s_3_0 s_3_2
        let s_3_3: i128 = ((s_3_0) * (s_3_2));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: write-var esize <= s_3_4
        fn_state.esize = s_3_4;
        // D s_3_6: read-var address:u32
        let s_3_6: u32 = fn_state.address;
        // D s_3_7: read-var ebytes:i64
        let s_3_7: i64 = fn_state.ebytes;
        // D s_3_8: call MemU_read(s_3_6, s_3_7)
        let s_3_8: Bits = MemU_read(state, tracer, s_3_6, s_3_7);
        // D s_3_9: write-var element <= s_3_8
        fn_state.element = s_3_8;
        // N s_3_10: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #64s : i
        let s_4_0: i128 = 64;
        // D s_4_1: read-var esize:i64
        let s_4_1: i64 = fn_state.esize;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: div s_4_0 s_4_2
        let s_4_3: i128 = ((s_4_0) / (s_4_2));
        // D s_4_4: cast reint s_4_3 -> i64
        let s_4_4: i64 = (s_4_3 as i64);
        // D s_4_5: cast zx s_4_4 -> i
        let s_4_5: i128 = (i128::try_from(s_4_4).unwrap());
        // D s_4_6: read-var element:bv
        let s_4_6: Bits = fn_state.element;
        // D s_4_7: cast reint s_4_5 -> u64
        let s_4_7: u64 = (s_4_5 as u64);
        // D s_4_8: call replicate_bits_borealis_internal(s_4_6, s_4_7)
        let s_4_8: Bits = replicate_bits_borealis_internal(state, tracer, s_4_6, s_4_7);
        // D s_4_9: cast reint s_4_8 -> u64
        let s_4_9: u64 = (s_4_8.value() as u64);
        // D s_4_10: write-var replicated_element <= s_4_9
        fn_state.replicated_element = s_4_9;
        // C s_4_11: const #0s : i64
        let s_4_11: i64 = 0;
        // C s_4_12: const #1s : i
        let s_4_12: i128 = 1;
        // D s_4_13: read-var regs:i64
        let s_4_13: i64 = fn_state.regs;
        // D s_4_14: cast zx s_4_13 -> i
        let s_4_14: i128 = (i128::try_from(s_4_13).unwrap());
        // D s_4_15: sub s_4_14 s_4_12
        let s_4_15: i128 = ((s_4_14) - (s_4_12));
        // D s_4_16: cast reint s_4_15 -> i64
        let s_4_16: i64 = (s_4_15 as i64);
        // D s_4_17: write-var gs#309427 <= s_4_16
        fn_state.gs_309427 = s_4_16;
        // D s_4_18: write-var r <= s_4_11
        fn_state.r = s_4_11;
        // N s_4_19: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var r:i64
        let s_5_0: i64 = fn_state.r;
        // D s_5_1: read-var gs#309427:i64
        let s_5_1: i64 = fn_state.gs_309427;
        // D s_5_2: cmp-gt s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) > (s_5_1));
        // N s_5_3: branch s_5_2 b7 b6
        if s_5_2 {
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
        // D s_6_2: read-var r:i64
        let s_6_2: i64 = fn_state.r;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: add s_6_1 s_6_3
        let s_6_4: i128 = (s_6_1 + s_6_3);
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // D s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_7: read-var replicated_element:u64
        let s_6_7: u64 = fn_state.replicated_element;
        // D s_6_8: call D_set(s_6_6, s_6_7)
        let s_6_8: () = D_set(state, tracer, s_6_6, s_6_7);
        // D s_6_9: read-var r:i64
        let s_6_9: i64 = fn_state.r;
        // C s_6_10: const #1s : i64
        let s_6_10: i64 = 1;
        // D s_6_11: add s_6_9 s_6_10
        let s_6_11: i64 = (s_6_9 + s_6_10);
        // D s_6_12: write-var r <= s_6_11
        fn_state.r = s_6_11;
        // N s_6_13: jump b5
        return block_5(state, tracer, fn_state);
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
        // D s_9_0: read-var register_index:u8
        let s_9_0: bool = fn_state.register_index;
        // N s_9_1: branch s_9_0 b11 b10
        if s_9_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var n:i64
        let s_10_0: i64 = fn_state.n;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: call R_read(s_10_1)
        let s_10_2: u32 = R_read(state, tracer, s_10_1);
        // D s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 32u16);
        // D s_10_4: read-var ebytes:i64
        let s_10_4: i64 = fn_state.ebytes;
        // D s_10_5: cast zx s_10_4 -> i
        let s_10_5: i128 = (i128::try_from(s_10_4).unwrap());
        // D s_10_6: cast cvt s_10_5 -> bv
        let s_10_6: Bits = Bits::new(s_10_5 as u128, 128);
        // D s_10_7: add s_10_3 s_10_6
        let s_10_7: Bits = (s_10_3 + s_10_6);
        // D s_10_8: cast reint s_10_7 -> u32
        let s_10_8: u32 = (s_10_7.value() as u32);
        // D s_10_9: read-var n:i64
        let s_10_9: i64 = fn_state.n;
        // D s_10_10: cast zx s_10_9 -> i
        let s_10_10: i128 = (i128::try_from(s_10_9).unwrap());
        // D s_10_11: call R_set(s_10_10, s_10_8)
        let s_10_11: () = R_set(state, tracer, s_10_10, s_10_8);
        // N s_10_12: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var n:i64
        let s_11_0: i64 = fn_state.n;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: call R_read(s_11_1)
        let s_11_2: u32 = R_read(state, tracer, s_11_1);
        // D s_11_3: read-var m:i64
        let s_11_3: i64 = fn_state.m;
        // D s_11_4: cast zx s_11_3 -> i
        let s_11_4: i128 = (i128::try_from(s_11_3).unwrap());
        // D s_11_5: call R_read(s_11_4)
        let s_11_5: u32 = R_read(state, tracer, s_11_4);
        // D s_11_6: cast zx s_11_2 -> bv
        let s_11_6: Bits = Bits::new(s_11_2 as u128, 32u16);
        // D s_11_7: cast zx s_11_5 -> bv
        let s_11_7: Bits = Bits::new(s_11_5 as u128, 32u16);
        // D s_11_8: add s_11_6 s_11_7
        let s_11_8: Bits = (s_11_6 + s_11_7);
        // D s_11_9: cast reint s_11_8 -> u32
        let s_11_9: u32 = (s_11_8.value() as u32);
        // D s_11_10: read-var n:i64
        let s_11_10: i64 = fn_state.n;
        // D s_11_11: cast zx s_11_10 -> i
        let s_11_11: i128 = (i128::try_from(s_11_10).unwrap());
        // D s_11_12: call R_set(s_11_11, s_11_9)
        let s_11_12: () = R_set(state, tracer, s_11_11, s_11_9);
        // N s_11_13: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var accdesc:struct
        let s_12_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_12_1: call AlignmentFault(s_12_0)
        let s_12_1: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_12_0);
        // D s_12_2: read-var address:u32
        let s_12_2: u32 = fn_state.address;
        // D s_12_3: call AArch32_Abort(s_12_2, s_12_1)
        let s_12_3: () = AArch32_Abort(state, tracer, s_12_2, s_12_1);
        // N s_12_4: jump b3
        return block_3(state, tracer, fn_state);
    }
}
