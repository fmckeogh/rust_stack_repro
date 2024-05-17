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
use AArch64_UnalignedAccessFaults::*;
use AArch64_MarkExclusiveVA::*;
use CreateAccDescExLDST::*;
use MarkExclusiveGlobal::*;
use ProcessorID::*;
use AArch64_Abort::*;
use IsAligned__1::*;
use IsFault::*;
use MarkExclusiveLocal::*;
use AArch64_TranslateAddress::*;
use AlignmentFault::*;
use common::*;
pub fn AArch64_SetExclusiveMonitors<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u64,
    size: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        accdesc: ProductType9878976b5bcce9c9,
        gs_21005: bool,
        ga_16417: ProductTypef170cab34335b70c,
        memaddrdesc: ProductTypece7c66ccb2cab13e,
        aligned: bool,
        address: u64,
        size: i128,
    }
    let fn_state = FunctionState {
        address,
        size,
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
        // C s_0_1: const #101160u : u32
        let s_0_1: u32 = 101160;
        // N s_0_2: write-reg s_0_1 <= s_0_0
        let s_0_2: () = {
            state.write_register::<bool>(s_0_1 as isize, s_0_0);
            tracer.write_register(s_0_1 as isize, s_0_0);
        };
        // C s_0_3: const #0u : u8
        let s_0_3: bool = false;
        // C s_0_4: const #0u : u8
        let s_0_4: bool = false;
        // C s_0_5: const #0u : u32
        let s_0_5: u32 = 0;
        // S s_0_6: call CreateAccDescExLDST(s_0_5, s_0_3, s_0_4)
        let s_0_6: ProductType9878976b5bcce9c9 = CreateAccDescExLDST(
            state,
            tracer,
            s_0_5,
            s_0_3,
            s_0_4,
        );
        // D s_0_7: write-var accdesc <= s_0_6
        fn_state.accdesc = s_0_6;
        // D s_0_8: read-var address:u64
        let s_0_8: u64 = fn_state.address;
        // D s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 64u16);
        // D s_0_10: read-var size:i
        let s_0_10: i128 = fn_state.size;
        // D s_0_11: call IsAligned__1(s_0_9, s_0_10)
        let s_0_11: bool = IsAligned__1(state, tracer, s_0_9, s_0_10);
        // D s_0_12: write-var aligned <= s_0_11
        fn_state.aligned = s_0_11;
        // D s_0_13: read-var aligned:u8
        let s_0_13: bool = fn_state.aligned;
        // D s_0_14: not s_0_13
        let s_0_14: bool = !s_0_13;
        // N s_0_15: branch s_0_14 b12 b1
        if s_0_14 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#21005 <= s_1_0
        fn_state.gs_21005 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#21005:u8
        let s_2_0: bool = fn_state.gs_21005;
        // N s_2_1: branch s_2_0 b11 b3
        if s_2_0 {
            return block_11(state, tracer, fn_state);
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
        // D s_4_0: read-var address:u64
        let s_4_0: u64 = fn_state.address;
        // D s_4_1: read-var accdesc:struct
        let s_4_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_4_2: read-var aligned:u8
        let s_4_2: bool = fn_state.aligned;
        // D s_4_3: read-var size:i
        let s_4_3: i128 = fn_state.size;
        // D s_4_4: call AArch64_TranslateAddress(s_4_0, s_4_1, s_4_2, s_4_3)
        let s_4_4: ProductTypece7c66ccb2cab13e = AArch64_TranslateAddress(
            state,
            tracer,
            s_4_0,
            s_4_1,
            s_4_2,
            s_4_3,
        );
        // D s_4_5: write-var memaddrdesc <= s_4_4
        fn_state.memaddrdesc = s_4_4;
        // N s_4_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var memaddrdesc:struct
        let s_5_0: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_5_1: call IsFault(s_5_0)
        let s_5_1: bool = IsFault(state, tracer, s_5_0);
        // N s_5_2: branch s_5_1 b10 b6
        if s_5_1 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var memaddrdesc.2:struct
        let s_6_0: ProductTypef170cab34335b70c = fn_state.memaddrdesc._2;
        // D s_6_1: write-var ga#16417 <= s_6_0
        fn_state.ga_16417 = s_6_0;
        // D s_6_2: read-var ga#16417.5:struct
        let s_6_2: u32 = fn_state.ga_16417._5;
        // C s_6_3: const #0u : u32
        let s_6_3: u32 = 0;
        // D s_6_4: cmp-eq s_6_2 s_6_3
        let s_6_4: bool = ((s_6_2) == (s_6_3));
        // N s_6_5: branch s_6_4 b9 b7
        if s_6_4 {
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
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var memaddrdesc.3:struct
        let s_8_0: ProductTypeda0231e9dc169f81 = fn_state.memaddrdesc._3;
        // C s_8_1: const #() : ()
        let s_8_1: () = ();
        // S s_8_2: call ProcessorID(s_8_1)
        let s_8_2: i128 = ProcessorID(state, tracer, s_8_1);
        // D s_8_3: read-var size:i
        let s_8_3: i128 = fn_state.size;
        // D s_8_4: call MarkExclusiveLocal(s_8_0, s_8_2, s_8_3)
        let s_8_4: () = MarkExclusiveLocal(state, tracer, s_8_0, s_8_2, s_8_3);
        // C s_8_5: const #() : ()
        let s_8_5: () = ();
        // S s_8_6: call ProcessorID(s_8_5)
        let s_8_6: i128 = ProcessorID(state, tracer, s_8_5);
        // D s_8_7: read-var address:u64
        let s_8_7: u64 = fn_state.address;
        // D s_8_8: read-var size:i
        let s_8_8: i128 = fn_state.size;
        // D s_8_9: call AArch64_MarkExclusiveVA(s_8_7, s_8_6, s_8_8)
        let s_8_9: () = AArch64_MarkExclusiveVA(state, tracer, s_8_7, s_8_6, s_8_8);
        // N s_8_10: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var memaddrdesc.3:struct
        let s_9_0: ProductTypeda0231e9dc169f81 = fn_state.memaddrdesc._3;
        // C s_9_1: const #() : ()
        let s_9_1: () = ();
        // S s_9_2: call ProcessorID(s_9_1)
        let s_9_2: i128 = ProcessorID(state, tracer, s_9_1);
        // D s_9_3: read-var size:i
        let s_9_3: i128 = fn_state.size;
        // D s_9_4: call MarkExclusiveGlobal(s_9_0, s_9_2, s_9_3)
        let s_9_4: () = MarkExclusiveGlobal(state, tracer, s_9_0, s_9_2, s_9_3);
        // N s_9_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var accdesc:struct
        let s_11_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_11_1: call AlignmentFault(s_11_0)
        let s_11_1: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_11_0);
        // D s_11_2: read-var address:u64
        let s_11_2: u64 = fn_state.address;
        // D s_11_3: call AArch64_Abort(s_11_2, s_11_1)
        let s_11_3: () = AArch64_Abort(state, tracer, s_11_2, s_11_1);
        // N s_11_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var accdesc:struct
        let s_12_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_12_1: read-var address:u64
        let s_12_1: u64 = fn_state.address;
        // D s_12_2: read-var size:i
        let s_12_2: i128 = fn_state.size;
        // D s_12_3: call AArch64_UnalignedAccessFaults(s_12_0, s_12_1, s_12_2)
        let s_12_3: bool = AArch64_UnalignedAccessFaults(
            state,
            tracer,
            s_12_0,
            s_12_1,
            s_12_2,
        );
        // D s_12_4: write-var gs#21005 <= s_12_3
        fn_state.gs_21005 = s_12_3;
        // N s_12_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
