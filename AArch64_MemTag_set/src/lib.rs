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
use IsFault__2::*;
use HaveMTE2Ext::*;
use HandleExternalWriteAbort::*;
use AArch64_AllocationTagAccessIsEnabled::*;
use AArch64_Abort::*;
use IsAligned__1::*;
use IsFault::*;
use PhysMemTagWrite::*;
use AArch64_TranslateAddress::*;
use AlignmentFault::*;
use common::*;
pub fn AArch64_MemTag_set<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u64,
    accdesc_in: ProductType9878976b5bcce9c9,
    value_name: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        accdesc: ProductType9878976b5bcce9c9,
        memstatusshadow_337: ProductTypef8c3639b88223255,
        ga_16238: ProductTypef170cab34335b70c,
        memaddrdesc: ProductTypece7c66ccb2cab13e,
        aligned: bool,
        gs_20816: bool,
        gs_20820: bool,
        address: u64,
        accdesc_in: ProductType9878976b5bcce9c9,
        value_name: u8,
    }
    let fn_state = FunctionState {
        address,
        accdesc_in,
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var accdesc_in.27:struct
        let s_0_0: bool = fn_state.accdesc_in._27;
        // N s_0_1: branch s_0_0 b20 b1
        if s_0_0 {
            return block_20(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#20816 <= s_1_0
        fn_state.gs_20816 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#20816:u8
        let s_2_0: bool = fn_state.gs_20816;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // D s_2_2: read-var accdesc_in:struct
        let s_2_2: ProductType9878976b5bcce9c9 = fn_state.accdesc_in;
        // D s_2_3: write-var accdesc <= s_2_2
        fn_state.accdesc = s_2_2;
        // D s_2_4: read-var address:u64
        let s_2_4: u64 = fn_state.address;
        // D s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 64u16);
        // C s_2_6: const #1344u : u32
        let s_2_6: u32 = 1344;
        // D s_2_7: read-reg s_2_6:i64
        let s_2_7: i64 = {
            let value = state.read_register::<i64>(s_2_6 as isize);
            tracer.read_register(s_2_6 as isize, value);
            value
        };
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (i128::try_from(s_2_7).unwrap());
        // D s_2_9: call IsAligned__1(s_2_5, s_2_8)
        let s_2_9: bool = IsAligned__1(state, tracer, s_2_5, s_2_8);
        // D s_2_10: write-var aligned <= s_2_9
        fn_state.aligned = s_2_9;
        // D s_2_11: read-var aligned:u8
        let s_2_11: bool = fn_state.aligned;
        // D s_2_12: not s_2_11
        let s_2_12: bool = !s_2_11;
        // N s_2_13: branch s_2_12 b19 b3
        if s_2_12 {
            return block_19(state, tracer, fn_state);
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
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call HaveMTE2Ext(s_4_0)
        let s_4_1: bool = HaveMTE2Ext(state, tracer, s_4_0);
        // N s_4_2: branch s_4_1 b18 b5
        if s_4_1 {
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
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #1344u : u32
        let s_6_0: u32 = 1344;
        // D s_6_1: read-reg s_6_0:i64
        let s_6_1: i64 = {
            let value = state.read_register::<i64>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: read-var address:u64
        let s_6_3: u64 = fn_state.address;
        // D s_6_4: read-var accdesc:struct
        let s_6_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_6_5: read-var aligned:u8
        let s_6_5: bool = fn_state.aligned;
        // D s_6_6: call AArch64_TranslateAddress(s_6_3, s_6_4, s_6_5, s_6_2)
        let s_6_6: ProductTypece7c66ccb2cab13e = AArch64_TranslateAddress(
            state,
            tracer,
            s_6_3,
            s_6_4,
            s_6_5,
            s_6_2,
        );
        // D s_6_7: write-var memaddrdesc <= s_6_6
        fn_state.memaddrdesc = s_6_6;
        // N s_6_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var memaddrdesc:struct
        let s_7_0: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_7_1: call IsFault(s_7_0)
        let s_7_1: bool = IsFault(state, tracer, s_7_0);
        // N s_7_2: branch s_7_1 b17 b8
        if s_7_1 {
            return block_17(state, tracer, fn_state);
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
        // D s_9_0: read-var accdesc.27:struct
        let s_9_0: bool = fn_state.accdesc._27;
        // N s_9_1: branch s_9_0 b16 b10
        if s_9_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#20820 <= s_10_0
        fn_state.gs_20820 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#20820:u8
        let s_11_0: bool = fn_state.gs_20820;
        // N s_11_1: branch s_11_0 b13 b12
        if s_11_0 {
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
        // D s_13_0: read-var memaddrdesc:struct
        let s_13_0: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_13_1: read-var accdesc:struct
        let s_13_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_13_2: read-var value_name:u8
        let s_13_2: u8 = fn_state.value_name;
        // D s_13_3: call PhysMemTagWrite(s_13_0, s_13_1, s_13_2)
        let s_13_3: ProductTypef8c3639b88223255 = PhysMemTagWrite(
            state,
            tracer,
            s_13_0,
            s_13_1,
            s_13_2,
        );
        // D s_13_4: write-var memstatusshadow#337 <= s_13_3
        fn_state.memstatusshadow_337 = s_13_3;
        // D s_13_5: read-var memstatusshadow#337:struct
        let s_13_5: ProductTypef8c3639b88223255 = fn_state.memstatusshadow_337;
        // D s_13_6: call IsFault__2(s_13_5)
        let s_13_6: bool = IsFault__2(state, tracer, s_13_5);
        // N s_13_7: branch s_13_6 b15 b14
        if s_13_6 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1s : i
        let s_15_0: i128 = 1;
        // D s_15_1: read-var memstatusshadow#337:struct
        let s_15_1: ProductTypef8c3639b88223255 = fn_state.memstatusshadow_337;
        // D s_15_2: read-var memaddrdesc:struct
        let s_15_2: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_15_3: read-var accdesc:struct
        let s_15_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_15_4: call HandleExternalWriteAbort(s_15_1, s_15_2, s_15_0, s_15_3)
        let s_15_4: () = HandleExternalWriteAbort(
            state,
            tracer,
            s_15_1,
            s_15_2,
            s_15_0,
            s_15_3,
        );
        // N s_15_5: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var memaddrdesc.2:struct
        let s_16_0: ProductTypef170cab34335b70c = fn_state.memaddrdesc._2;
        // D s_16_1: write-var ga#16238 <= s_16_0
        fn_state.ga_16238 = s_16_0;
        // D s_16_2: read-var ga#16238.6:struct
        let s_16_2: u32 = fn_state.ga_16238._6;
        // C s_16_3: const #1u : u32
        let s_16_3: u32 = 1;
        // D s_16_4: cmp-eq s_16_2 s_16_3
        let s_16_4: bool = ((s_16_2) == (s_16_3));
        // D s_16_5: write-var gs#20820 <= s_16_4
        fn_state.gs_20820 = s_16_4;
        // N s_16_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var memaddrdesc.0:struct
        let s_17_0: ProductType1d757adad216cdef = fn_state.memaddrdesc._0;
        // D s_17_1: read-var address:u64
        let s_17_1: u64 = fn_state.address;
        // D s_17_2: call AArch64_Abort(s_17_1, s_17_0)
        let s_17_2: () = AArch64_Abort(state, tracer, s_17_1, s_17_0);
        // N s_17_3: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var accdesc.8:struct
        let s_18_0: u8 = fn_state.accdesc._8;
        // D s_18_1: call AArch64_AllocationTagAccessIsEnabled(s_18_0)
        let s_18_1: bool = AArch64_AllocationTagAccessIsEnabled(state, tracer, s_18_0);
        // D s_18_2: write-var accdesc.27 <= s_18_1
        fn_state.accdesc._27 = s_18_1;
        // N s_18_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var accdesc:struct
        let s_19_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_19_1: call AlignmentFault(s_19_0)
        let s_19_1: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_19_0);
        // D s_19_2: read-var address:u64
        let s_19_2: u64 = fn_state.address;
        // D s_19_3: call AArch64_Abort(s_19_2, s_19_1)
        let s_19_3: () = AArch64_Abort(state, tracer, s_19_2, s_19_1);
        // N s_19_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var accdesc_in.28:struct
        let s_20_0: bool = fn_state.accdesc_in._28;
        // D s_20_1: not s_20_0
        let s_20_1: bool = !s_20_0;
        // D s_20_2: write-var gs#20816 <= s_20_1
        fn_state.gs_20816 = s_20_1;
        // N s_20_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
