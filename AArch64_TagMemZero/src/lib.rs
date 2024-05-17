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
use IsDebugException::*;
use IsFault__2::*;
use AlignmentFault::*;
use HaveMTE2Ext::*;
use HandleExternalWriteAbort::*;
use AArch64_AllocationTagAccessIsEnabled::*;
use AArch64_Abort::*;
use IsAligned__1::*;
use IsFault::*;
use PhysMemTagWrite::*;
use AArch64_TranslateAddress::*;
use AArch64_AllocationTagFromAddress::*;
use common::*;
pub fn AArch64_TagMemZero<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regval: u64,
    vaddress: u64,
    accdesc_in: ProductType9878976b5bcce9c9,
    size: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        tag: u8,
        gs_20989: bool,
        ga_16406: ProductTypef170cab34335b70c,
        memstatus: ProductTypef8c3639b88223255,
        gs_20993: i64,
        count: i128,
        accdesc: ProductType9878976b5bcce9c9,
        ga_16411: ProductTypeda0231e9dc169f81,
        memaddrdesc: ProductTypece7c66ccb2cab13e,
        aligned: bool,
        i: i64,
        gs_20984: bool,
        regval: u64,
        vaddress: u64,
        accdesc_in: ProductType9878976b5bcce9c9,
        size: i128,
    }
    let fn_state = FunctionState {
        regval,
        vaddress,
        accdesc_in,
        size,
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
        // N s_0_1: branch s_0_0 b26 b1
        if s_0_0 {
            return block_26(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#20984 <= s_1_0
        fn_state.gs_20984 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#20984:u8
        let s_2_0: bool = fn_state.gs_20984;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // D s_2_2: read-var accdesc_in:struct
        let s_2_2: ProductType9878976b5bcce9c9 = fn_state.accdesc_in;
        // D s_2_3: write-var accdesc <= s_2_2
        fn_state.accdesc = s_2_2;
        // C s_2_4: const #456u : u32
        let s_2_4: u32 = 456;
        // D s_2_5: read-reg s_2_4:i64
        let s_2_5: i64 = {
            let value = state.read_register::<i64>(s_2_4 as isize);
            tracer.read_register(s_2_4 as isize, value);
            value
        };
        // D s_2_6: cast zx s_2_5 -> i
        let s_2_6: i128 = (i128::try_from(s_2_5).unwrap());
        // D s_2_7: read-var size:i
        let s_2_7: i128 = fn_state.size;
        // D s_2_8: lsr s_2_7 s_2_6
        let s_2_8: i128 = s_2_7 >> s_2_6;
        // D s_2_9: write-var count <= s_2_8
        fn_state.count = s_2_8;
        // D s_2_10: read-var vaddress:u64
        let s_2_10: u64 = fn_state.vaddress;
        // D s_2_11: call AArch64_AllocationTagFromAddress(s_2_10)
        let s_2_11: u8 = AArch64_AllocationTagFromAddress(state, tracer, s_2_10);
        // D s_2_12: write-var tag <= s_2_11
        fn_state.tag = s_2_11;
        // D s_2_13: read-var vaddress:u64
        let s_2_13: u64 = fn_state.vaddress;
        // D s_2_14: cast zx s_2_13 -> bv
        let s_2_14: Bits = Bits::new(s_2_13 as u128, 64u16);
        // C s_2_15: const #1344u : u32
        let s_2_15: u32 = 1344;
        // D s_2_16: read-reg s_2_15:i64
        let s_2_16: i64 = {
            let value = state.read_register::<i64>(s_2_15 as isize);
            tracer.read_register(s_2_15 as isize, value);
            value
        };
        // D s_2_17: cast zx s_2_16 -> i
        let s_2_17: i128 = (i128::try_from(s_2_16).unwrap());
        // D s_2_18: call IsAligned__1(s_2_14, s_2_17)
        let s_2_18: bool = IsAligned__1(state, tracer, s_2_14, s_2_17);
        // D s_2_19: write-var aligned <= s_2_18
        fn_state.aligned = s_2_18;
        // D s_2_20: read-var aligned:u8
        let s_2_20: bool = fn_state.aligned;
        // D s_2_21: not s_2_20
        let s_2_21: bool = !s_2_20;
        // N s_2_22: branch s_2_21 b25 b3
        if s_2_21 {
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
        // N s_4_2: branch s_4_1 b24 b5
        if s_4_1 {
            return block_24(state, tracer, fn_state);
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
        // D s_6_0: read-var vaddress:u64
        let s_6_0: u64 = fn_state.vaddress;
        // D s_6_1: read-var accdesc:struct
        let s_6_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_6_2: read-var aligned:u8
        let s_6_2: bool = fn_state.aligned;
        // D s_6_3: read-var size:i
        let s_6_3: i128 = fn_state.size;
        // D s_6_4: call AArch64_TranslateAddress(s_6_0, s_6_1, s_6_2, s_6_3)
        let s_6_4: ProductTypece7c66ccb2cab13e = AArch64_TranslateAddress(
            state,
            tracer,
            s_6_0,
            s_6_1,
            s_6_2,
            s_6_3,
        );
        // D s_6_5: write-var memaddrdesc <= s_6_4
        fn_state.memaddrdesc = s_6_4;
        // N s_6_6: jump b7
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
        // N s_7_2: branch s_7_1 b21 b8
        if s_7_1 {
            return block_21(state, tracer, fn_state);
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
        // D s_9_1: not s_9_0
        let s_9_1: bool = !s_9_0;
        // N s_9_2: branch s_9_1 b20 b10
        if s_9_1 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var memaddrdesc.2:struct
        let s_10_0: ProductTypef170cab34335b70c = fn_state.memaddrdesc._2;
        // D s_10_1: write-var ga#16406 <= s_10_0
        fn_state.ga_16406 = s_10_0;
        // D s_10_2: read-var ga#16406.6:struct
        let s_10_2: u32 = fn_state.ga_16406._6;
        // C s_10_3: const #1u : u32
        let s_10_3: u32 = 1;
        // D s_10_4: cmp-eq s_10_2 s_10_3
        let s_10_4: bool = ((s_10_2) == (s_10_3));
        // D s_10_5: write-var gs#20989 <= s_10_4
        fn_state.gs_20989 = s_10_4;
        // N s_10_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#20989:u8
        let s_11_0: bool = fn_state.gs_20989;
        // N s_11_1: branch s_11_0 b19 b12
        if s_11_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0s : i64
        let s_12_0: i64 = 0;
        // C s_12_1: const #1s : i
        let s_12_1: i128 = 1;
        // D s_12_2: read-var count:i
        let s_12_2: i128 = fn_state.count;
        // D s_12_3: sub s_12_2 s_12_1
        let s_12_3: i128 = ((s_12_2) - (s_12_1));
        // D s_12_4: cast reint s_12_3 -> i64
        let s_12_4: i64 = (s_12_3 as i64);
        // D s_12_5: write-var gs#20993 <= s_12_4
        fn_state.gs_20993 = s_12_4;
        // D s_12_6: write-var i <= s_12_0
        fn_state.i = s_12_0;
        // N s_12_7: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var i:i64
        let s_13_0: i64 = fn_state.i;
        // D s_13_1: read-var gs#20993:i64
        let s_13_1: i64 = fn_state.gs_20993;
        // D s_13_2: cmp-gt s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) > (s_13_1));
        // N s_13_3: branch s_13_2 b18 b14
        if s_13_2 {
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
        // D s_14_0: read-var memaddrdesc:struct
        let s_14_0: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_14_1: read-var accdesc:struct
        let s_14_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_14_2: read-var tag:u8
        let s_14_2: u8 = fn_state.tag;
        // D s_14_3: call PhysMemTagWrite(s_14_0, s_14_1, s_14_2)
        let s_14_3: ProductTypef8c3639b88223255 = PhysMemTagWrite(
            state,
            tracer,
            s_14_0,
            s_14_1,
            s_14_2,
        );
        // D s_14_4: write-var memstatus <= s_14_3
        fn_state.memstatus = s_14_3;
        // D s_14_5: read-var memstatus:struct
        let s_14_5: ProductTypef8c3639b88223255 = fn_state.memstatus;
        // D s_14_6: call IsFault__2(s_14_5)
        let s_14_6: bool = IsFault__2(state, tracer, s_14_5);
        // N s_14_7: branch s_14_6 b17 b15
        if s_14_6 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var memaddrdesc.3:struct
        let s_16_0: ProductTypeda0231e9dc169f81 = fn_state.memaddrdesc._3;
        // D s_16_1: write-var ga#16411 <= s_16_0
        fn_state.ga_16411 = s_16_0;
        // D s_16_2: read-var ga#16411.0:struct
        let s_16_2: u64 = fn_state.ga_16411._0;
        // D s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 56u16);
        // C s_16_4: const #1344u : u32
        let s_16_4: u32 = 1344;
        // D s_16_5: read-reg s_16_4:i64
        let s_16_5: i64 = {
            let value = state.read_register::<i64>(s_16_4 as isize);
            tracer.read_register(s_16_4 as isize, value);
            value
        };
        // D s_16_6: cast zx s_16_5 -> i
        let s_16_6: i128 = (i128::try_from(s_16_5).unwrap());
        // D s_16_7: cast cvt s_16_6 -> bv
        let s_16_7: Bits = Bits::new(s_16_6 as u128, 128);
        // D s_16_8: add s_16_3 s_16_7
        let s_16_8: Bits = (s_16_3 + s_16_7);
        // D s_16_9: cast reint s_16_8 -> u56
        let s_16_9: u64 = (s_16_8.value() as u64);
        // D s_16_10: write-var memaddrdesc.3.0 <= s_16_9
        fn_state.memaddrdesc._3._0 = s_16_9;
        // D s_16_11: read-var i:i64
        let s_16_11: i64 = fn_state.i;
        // C s_16_12: const #1s : i64
        let s_16_12: i64 = 1;
        // D s_16_13: add s_16_11 s_16_12
        let s_16_13: i64 = (s_16_11 + s_16_12);
        // D s_16_14: write-var i <= s_16_13
        fn_state.i = s_16_13;
        // N s_16_15: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1s : i
        let s_17_0: i128 = 1;
        // D s_17_1: read-var memstatus:struct
        let s_17_1: ProductTypef8c3639b88223255 = fn_state.memstatus;
        // D s_17_2: read-var memaddrdesc:struct
        let s_17_2: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_17_3: read-var accdesc:struct
        let s_17_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_17_4: call HandleExternalWriteAbort(s_17_1, s_17_2, s_17_0, s_17_3)
        let s_17_4: () = HandleExternalWriteAbort(
            state,
            tracer,
            s_17_1,
            s_17_2,
            s_17_0,
            s_17_3,
        );
        // N s_17_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#20989 <= s_20_0
        fn_state.gs_20989 = s_20_0;
        // N s_20_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var memaddrdesc.0:struct
        let s_21_0: ProductType1d757adad216cdef = fn_state.memaddrdesc._0;
        // D s_21_1: call IsDebugException(s_21_0)
        let s_21_1: bool = IsDebugException(state, tracer, s_21_0);
        // N s_21_2: branch s_21_1 b23 b22
        if s_21_1 {
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
        // D s_22_0: read-var memaddrdesc.0:struct
        let s_22_0: ProductType1d757adad216cdef = fn_state.memaddrdesc._0;
        // D s_22_1: read-var regval:u64
        let s_22_1: u64 = fn_state.regval;
        // D s_22_2: call AArch64_Abort(s_22_1, s_22_0)
        let s_22_2: () = AArch64_Abort(state, tracer, s_22_1, s_22_0);
        // N s_22_3: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var memaddrdesc.0:struct
        let s_23_0: ProductType1d757adad216cdef = fn_state.memaddrdesc._0;
        // D s_23_1: read-var vaddress:u64
        let s_23_1: u64 = fn_state.vaddress;
        // D s_23_2: call AArch64_Abort(s_23_1, s_23_0)
        let s_23_2: () = AArch64_Abort(state, tracer, s_23_1, s_23_0);
        // N s_23_3: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var accdesc.8:struct
        let s_24_0: u8 = fn_state.accdesc._8;
        // D s_24_1: call AArch64_AllocationTagAccessIsEnabled(s_24_0)
        let s_24_1: bool = AArch64_AllocationTagAccessIsEnabled(state, tracer, s_24_0);
        // D s_24_2: write-var accdesc.27 <= s_24_1
        fn_state.accdesc._27 = s_24_1;
        // N s_24_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var accdesc:struct
        let s_25_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_25_1: call AlignmentFault(s_25_0)
        let s_25_1: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_25_0);
        // D s_25_2: read-var vaddress:u64
        let s_25_2: u64 = fn_state.vaddress;
        // D s_25_3: call AArch64_Abort(s_25_2, s_25_1)
        let s_25_3: () = AArch64_Abort(state, tracer, s_25_2, s_25_1);
        // N s_25_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var accdesc_in.28:struct
        let s_26_0: bool = fn_state.accdesc_in._28;
        // D s_26_1: not s_26_0
        let s_26_1: bool = !s_26_0;
        // D s_26_2: write-var gs#20984 <= s_26_1
        fn_state.gs_20984 = s_26_1;
        // N s_26_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
