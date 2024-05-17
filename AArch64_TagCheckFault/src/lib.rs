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
use HaveMTEAsymFaultExt::*;
use AArch64_ReportTagCheckFault::*;
use NoFault__1::*;
use AArch64_EffectiveTCF::*;
use AArch64_RaiseTagCheckFault::*;
use HaveMTEAsyncExt::*;
use common::*;
pub fn AArch64_TagCheckFault<T: Tracer>(
    state: &mut State,
    tracer: &T,
    vaddress: u64,
    accdesc: ProductType9878976b5bcce9c9,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        tcf: u8,
        fault: ProductType1d757adad216cdef,
        vaddress: u64,
        accdesc: ProductType9878976b5bcce9c9,
    }
    let fn_state = FunctionState {
        vaddress,
        accdesc,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var accdesc.8:struct
        let s_0_0: u8 = fn_state.accdesc._8;
        // D s_0_1: call AArch64_EffectiveTCF(s_0_0)
        let s_0_1: u8 = AArch64_EffectiveTCF(state, tracer, s_0_0);
        // D s_0_2: write-var tcf <= s_0_1
        fn_state.tcf = s_0_1;
        // D s_0_3: read-var accdesc:struct
        let s_0_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_0_4: call NoFault__1(s_0_3)
        let s_0_4: ProductType1d757adad216cdef = NoFault__1(state, tracer, s_0_3);
        // D s_0_5: write-var fault <= s_0_4
        fn_state.fault = s_0_4;
        // C s_0_6: const #16u : u32
        let s_0_6: u32 = 16;
        // D s_0_7: write-var fault.16 <= s_0_6
        fn_state.fault._16 = s_0_6;
        // D s_0_8: read-var tcf:u8
        let s_0_8: u8 = fn_state.tcf;
        // D s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 2u16);
        // C s_0_10: const #0u : u8
        let s_0_10: u8 = 0;
        // C s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 2u16);
        // D s_0_12: cmp-eq s_0_9 s_0_11
        let s_0_12: bool = ((s_0_9) == (s_0_11));
        // D s_0_13: not s_0_12
        let s_0_13: bool = !s_0_12;
        // N s_0_14: branch s_0_13 b2 b1
        if s_0_13 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var tcf:u8
        let s_2_0: u8 = fn_state.tcf;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #1u : u8
        let s_2_2: u8 = 1;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // D s_2_5: not s_2_4
        let s_2_5: bool = !s_2_4;
        // N s_2_6: branch s_2_5 b4 b3
        if s_2_5 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var vaddress:u64
        let s_3_0: u64 = fn_state.vaddress;
        // D s_3_1: read-var fault:struct
        let s_3_1: ProductType1d757adad216cdef = fn_state.fault;
        // D s_3_2: call AArch64_RaiseTagCheckFault(s_3_0, s_3_1)
        let s_3_2: () = AArch64_RaiseTagCheckFault(state, tracer, s_3_0, s_3_1);
        // N s_3_3: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var tcf:u8
        let s_4_0: u8 = fn_state.tcf;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // C s_4_2: const #2u : u8
        let s_4_2: u8 = 2;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 2u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // D s_4_5: not s_4_4
        let s_4_5: bool = !s_4_4;
        // N s_4_6: branch s_4_5 b8 b5
        if s_4_5 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call HaveMTEAsyncExt(s_5_0)
        let s_5_1: bool = HaveMTEAsyncExt(state, tracer, s_5_0);
        // N s_5_2: branch s_5_1 b7 b6
        if s_5_1 {
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
        // N s_6_0: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var accdesc.8:struct
        let s_7_0: u8 = fn_state.accdesc._8;
        // C s_7_1: const #55s : i
        let s_7_1: i128 = 55;
        // D s_7_2: read-var vaddress:u64
        let s_7_2: u64 = fn_state.vaddress;
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 64u16);
        // C s_7_4: const #1u : u64
        let s_7_4: u64 = 1;
        // D s_7_5: bit-extract s_7_3 s_7_1 s_7_4
        let s_7_5: Bits = (Bits::new(
            ((s_7_3) >> (s_7_1)).value(),
            u16::try_from(s_7_4).unwrap(),
        ));
        // D s_7_6: cast reint s_7_5 -> u8
        let s_7_6: bool = ((s_7_5.value()) != 0);
        // C s_7_7: const #0s : i
        let s_7_7: i128 = 0;
        // C s_7_8: const #0u : u64
        let s_7_8: u64 = 0;
        // D s_7_9: cast zx s_7_6 -> u64
        let s_7_9: u64 = (s_7_6 as u64);
        // C s_7_10: const #1u : u64
        let s_7_10: u64 = 1;
        // D s_7_11: and s_7_9 s_7_10
        let s_7_11: u64 = ((s_7_9) & (s_7_10));
        // D s_7_12: cmp-eq s_7_11 s_7_10
        let s_7_12: bool = ((s_7_11) == (s_7_10));
        // D s_7_13: lsl s_7_9 s_7_7
        let s_7_13: u64 = s_7_9 << s_7_7;
        // D s_7_14: or s_7_8 s_7_13
        let s_7_14: u64 = ((s_7_8) | (s_7_13));
        // D s_7_15: cmpl s_7_13
        let s_7_15: u64 = !s_7_13;
        // D s_7_16: and s_7_8 s_7_15
        let s_7_16: u64 = ((s_7_8) & (s_7_15));
        // D s_7_17: select s_7_12 s_7_14 s_7_16
        let s_7_17: u64 = if s_7_12 { s_7_14 } else { s_7_16 };
        // D s_7_18: cast trunc s_7_17 -> u8
        let s_7_18: bool = ((s_7_17) != 0);
        // D s_7_19: call AArch64_ReportTagCheckFault(s_7_0, s_7_18)
        let s_7_19: () = AArch64_ReportTagCheckFault(state, tracer, s_7_0, s_7_18);
        // N s_7_20: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call HaveMTEAsymFaultExt(s_8_0)
        let s_8_1: bool = HaveMTEAsymFaultExt(state, tracer, s_8_0);
        // N s_8_2: branch s_8_1 b10 b9
        if s_8_1 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var accdesc.23:struct
        let s_10_0: bool = fn_state.accdesc._23;
        // N s_10_1: branch s_10_0 b12 b11
        if s_10_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var accdesc.8:struct
        let s_11_0: u8 = fn_state.accdesc._8;
        // C s_11_1: const #55s : i
        let s_11_1: i128 = 55;
        // D s_11_2: read-var vaddress:u64
        let s_11_2: u64 = fn_state.vaddress;
        // D s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 64u16);
        // C s_11_4: const #1u : u64
        let s_11_4: u64 = 1;
        // D s_11_5: bit-extract s_11_3 s_11_1 s_11_4
        let s_11_5: Bits = (Bits::new(
            ((s_11_3) >> (s_11_1)).value(),
            u16::try_from(s_11_4).unwrap(),
        ));
        // D s_11_6: cast reint s_11_5 -> u8
        let s_11_6: bool = ((s_11_5.value()) != 0);
        // C s_11_7: const #0s : i
        let s_11_7: i128 = 0;
        // C s_11_8: const #0u : u64
        let s_11_8: u64 = 0;
        // D s_11_9: cast zx s_11_6 -> u64
        let s_11_9: u64 = (s_11_6 as u64);
        // C s_11_10: const #1u : u64
        let s_11_10: u64 = 1;
        // D s_11_11: and s_11_9 s_11_10
        let s_11_11: u64 = ((s_11_9) & (s_11_10));
        // D s_11_12: cmp-eq s_11_11 s_11_10
        let s_11_12: bool = ((s_11_11) == (s_11_10));
        // D s_11_13: lsl s_11_9 s_11_7
        let s_11_13: u64 = s_11_9 << s_11_7;
        // D s_11_14: or s_11_8 s_11_13
        let s_11_14: u64 = ((s_11_8) | (s_11_13));
        // D s_11_15: cmpl s_11_13
        let s_11_15: u64 = !s_11_13;
        // D s_11_16: and s_11_8 s_11_15
        let s_11_16: u64 = ((s_11_8) & (s_11_15));
        // D s_11_17: select s_11_12 s_11_14 s_11_16
        let s_11_17: u64 = if s_11_12 { s_11_14 } else { s_11_16 };
        // D s_11_18: cast trunc s_11_17 -> u8
        let s_11_18: bool = ((s_11_17) != 0);
        // D s_11_19: call AArch64_ReportTagCheckFault(s_11_0, s_11_18)
        let s_11_19: () = AArch64_ReportTagCheckFault(state, tracer, s_11_0, s_11_18);
        // N s_11_20: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var vaddress:u64
        let s_12_0: u64 = fn_state.vaddress;
        // D s_12_1: read-var fault:struct
        let s_12_1: ProductType1d757adad216cdef = fn_state.fault;
        // D s_12_2: call AArch64_RaiseTagCheckFault(s_12_0, s_12_1)
        let s_12_2: () = AArch64_RaiseTagCheckFault(state, tracer, s_12_0, s_12_1);
        // N s_12_3: return
        return;
    }
}
